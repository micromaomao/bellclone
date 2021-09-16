use std::collections::{hash_map::Entry, HashMap};

use crate::{
  ec::EcCtx,
  enc::{encode_player_position, encode_player_score},
  log,
  render::view::ViewportInfo,
};
use crate::{
  ec::{
    components::{
      bell::OurJumpableBell,
      player::{OurPlayer, OurPlayerState},
      DrawImage,
    },
    BlurFlags,
  },
  global,
  render::{view::view_matrix, ViewportSize},
};
use game_core::{STAGE_MIN_HEIGHT, STAGE_WIDTH, dec::{parse_entity_id, parse_mat4}, ec::{DeltaTime, components::{EntityId, bell::BellComponent, bird::{Bird, Direction}, physics::Velocity, player::PlayerComponent, transform::WorldSpaceTransform}, systems::{create_bell::CreateBellSystemControl, create_bird::CreateBirdSystemController}}};
use glam::f32::*;
use protocol::{
  clientmsg_generated::ClientMessage,
  flatbuffers::{FlatBufferBuilder, WIPOffset},
  servermsg_generated::{ServerMessage, ServerMessageInner},
};
use specs::{Builder, Entity, EntityBuilder, Join, WorldExt};
pub mod player;
use player::{create_our_player, create_remote_player, delete_player};

pub struct WorldManager {
  me: Option<Entity>,
  camera_y: f32,
  /// only used for tracking server bells
  entityid_map: HashMap<EntityId, Entity>,
  state: GameState,
}

pub const CAMERA_OFFSET_Y: f32 = -4f32;
pub const CAMERA_INIT_Y: f32 = -2f32;
pub const CAMERA_TARGET_EPSILON: f32 = 0.1f32;
pub const CAMERA_SPEED_MUL: f32 = 4f32;
pub const CAMERA_SWITCH_TO_GROUND_EARLY_PERIOD: f32 = 0.5f32; // secs

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum GameState {
  Connecting,
  Online,
  Offline,
}

impl WorldManager {
  pub fn new(ec: &mut EcCtx) -> Self {
    ec.world.maintain();
    // todo
    WorldManager {
      me: None,
      camera_y: CAMERA_INIT_Y,
      entityid_map: HashMap::new(),
      state: GameState::Connecting,
    }
  }

  pub fn init_common(&mut self, ec: &mut EcCtx) {
  }

  pub fn init_offline(&mut self, ec: &mut EcCtx) {
    ec.world.write_resource::<CreateBellSystemControl>().enabled = true;
    ec.world.write_resource::<CreateBirdSystemController>().enabled = true;
    self.init_common(ec);
    if self.me.is_none() {
      self.me = Some(create_our_player(ec));
    } // otherwise we can just keep the current player position and "switch seamlessly" from online to offline.
    ec.world.maintain();
    self.state = GameState::Offline;
  }

  pub fn show_connection_error(&mut self, _ec: &mut EcCtx) {
    // todo
  }

  pub fn init_online(&mut self, ec: &mut EcCtx) {
    ec.world.delete_all();
    self.me = None;
    self.init_common(ec);
    self.me = Some(create_our_player(ec));
    self.state = GameState::Online;
  }

  pub fn update(&mut self, ec: &mut EcCtx) {
    match self.state {
      GameState::Connecting => {}
      GameState::Offline => {
        self.offline_update(ec);
      }
      GameState::Online => {
        // todo
      }
    }
  }

  pub fn offline_update(&mut self, ec: &mut EcCtx) {}

  pub fn calculate_camera(&mut self, ec: &EcCtx, viewport_size: ViewportSize) -> ViewportInfo {
    let w = &ec.world;
    if self.me.is_none() {
      self.camera_y = CAMERA_INIT_Y;
      return view_matrix(viewport_size, self.camera_y);
    }
    let me = self.me.unwrap();
    let our_player_storage = w.read_storage::<OurPlayer>();
    let player_state = our_player_storage.get(me).unwrap();
    let mut cam_y = self.camera_y;
    let player_y = w
      .read_storage::<WorldSpaceTransform>()
      .get(me)
      .unwrap()
      .position()
      .y;
    let player_v = w.read_storage::<Velocity>().get(me).unwrap().0.y;
    let mut blur_flags = w.write_resource::<BlurFlags>();
    blur_flags.motion_blur_dy = 0f32;
    if player_state.state == OurPlayerState::Falling
      && player_y < {
        let ViewportSize { width, height, .. } = viewport_size;
        // calculate threshold y before camera cut to ground.
        let mut visible_height = STAGE_MIN_HEIGHT;
        if height > width {
          visible_height += (height as f32 / width as f32 - 1f32) * (STAGE_WIDTH);
        }
        visible_height - player_v * CAMERA_SWITCH_TO_GROUND_EARLY_PERIOD
      }
    {
      cam_y = CAMERA_INIT_Y;
    } else {
      let target_y = match player_state.state {
        OurPlayerState::Falling | OurPlayerState::Flying => player_y + CAMERA_OFFSET_Y,
        OurPlayerState::NotStarted => CAMERA_INIT_Y,
      };
      let dt = ec.world.read_resource::<DeltaTime>().as_secs_f32();
      let diff = target_y - cam_y;
      if diff.abs() > CAMERA_TARGET_EPSILON {
        let v = CAMERA_SPEED_MUL * diff;
        cam_y += dt * v;
        if v < -5f32 {
          blur_flags.motion_blur_dy = -v;
        }
      }
    }
    // cam_y += (player_v * 0.5).min(CAMERA_MAX_SPEED) * dt;
    self.camera_y = cam_y;
    view_matrix(viewport_size, cam_y)
  }

  pub fn process_msg(&mut self, ec: &mut EcCtx, msg: ServerMessage<'_>) {
    if self.state != GameState::Online {
      return;
    }
    match msg.msg_type() {
      ServerMessageInner::PlayerUpdate => {
        let msg = msg.msg_as_player_update().unwrap();
        let player_id = parse_entity_id(msg.id().unwrap());
        let pos = msg.pos().unwrap();
        let vel = msg.vel().unwrap();
        let ent = match self.entityid_map.entry(player_id) {
          Entry::Occupied(ent) => *ent.get(),
          Entry::Vacant(slot) => {
            let new_ent = create_remote_player(ec);
            slot.insert(new_ent);
            new_ent
          }
        };
        if ec.world.read_storage::<OurPlayer>().contains(ent) {
          return;
        }
        ec.world
          .write_storage::<WorldSpaceTransform>()
          .insert(
            ent,
            WorldSpaceTransform::from_pos(Vec3::new(pos.x(), pos.y(), 0f32)),
          )
          .unwrap();
        ec.world
          .write_storage::<Velocity>()
          .insert(ent, Velocity(Vec2::new(vel.x(), vel.y())))
          .unwrap();
      }
      ServerMessageInner::PlayerDelete => {
        let msg = msg.msg_as_player_delete().unwrap();
        let uuid = parse_entity_id(msg.id().unwrap());
        match self.entityid_map.get(&uuid) {
          None => {}
          Some(&ent) => {
            delete_player(ec, ent);
          }
        }
      }
      ServerMessageInner::Bells => {
        use game_core::ec::systems::create_bell::BELL_SIZE; // TODO
        let msg = msg.msg_as_bells().unwrap();
        let w = &mut ec.world;
        for b in msg.bells().unwrap() {
          w.create_entity()
            .with(BellComponent { size: BELL_SIZE })
            .with(
              WorldSpaceTransform::from_pos(Vec3::new(
                b.pos().unwrap().x(),
                b.pos().unwrap().y(),
                0f32,
              ))
              .add(Mat4::from_scale(Vec3::new(BELL_SIZE, BELL_SIZE, 1f32))),
            )
            .with(Velocity(Vec2::new(
              b.vel().unwrap().x(),
              b.vel().unwrap().y(),
            )))
            .build();
        }
      }
      ServerMessageInner::YourIDIs => {
        let msg = msg.msg_as_your_idis().unwrap();
        let id = parse_entity_id(msg.id().unwrap());
        log!("Our id is {}", id.0);
        let ops = ec.world.read_storage::<OurPlayer>();
        let ent = ec.world.entities();
        let mut our_player_ent = None;
        for (_, ent) in (&ops, &ent).join() {
          our_player_ent = Some(ent);
          break;
        }
        if let Some(&e) = self.entityid_map.get(&id) {
          ent.delete(e);
        }
        self.entityid_map.insert(id, our_player_ent.unwrap());
      },
      ServerMessageInner::Birds => {
        let msg = msg.msg_as_birds().unwrap();
        let w = &mut ec.world;
        let ents = w.entities();
        let mut birdc = w.write_storage::<Bird>();
        let mut tr = w.write_storage::<WorldSpaceTransform>();
        let mut jumpc = w.write_storage::<OurJumpableBell>();
        for b in msg.birds().unwrap() {
          let ent = *self.entityid_map.entry(parse_entity_id(b.id().unwrap())).or_insert_with(|| ents.create());
          if !birdc.contains(ent) {
            jumpc.insert(ent, OurJumpableBell);
          }
          birdc.insert(ent, Bird {
            direction: match b.dir_is_right() {
              true => Direction::RIGHT,
              false => Direction::LEFT
            },
            turning: b.turning()
          });
          tr.insert(ent, WorldSpaceTransform(parse_mat4(b.transform().unwrap())));
        }
      },
      _ => unreachable!(),
    }
  }

  pub fn get_regular_updates<'a, F>(
    &mut self,
    ec: &mut EcCtx,
    fbb: &mut FlatBufferBuilder<'a>,
    mut send: F,
  ) where
    F: FnMut(WIPOffset<ClientMessage<'a>>, &mut FlatBufferBuilder<'a>),
  {
    if self.me.is_none() {
      return;
    }
    let me = self.me.unwrap();
    let trs = ec.world.read_storage::<WorldSpaceTransform>();
    let vels = ec.world.read_storage::<Velocity>();
    let pls = ec.world.read_storage::<PlayerComponent>();
    let tr = trs.get(me).unwrap();
    let vel = vels.get(me).unwrap();
    let score = pls.get(me).unwrap().score;
    send(encode_player_position(fbb, tr, vel), fbb);
    send(encode_player_score(fbb, score), fbb);
  }
}

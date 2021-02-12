use std::collections::{hash_map::Entry, HashMap};

use crate::{ec::EcCtx, render::view::ViewportInfo};
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
use game_core::{STAGE_MIN_HEIGHT, STAGE_WIDTH, dec::parse_entity_id, ec::{DeltaTime, components::{EntityId, bell::BellComponent, physics::Velocity, transform::WorldSpaceTransform}}, gen::BellGenContext};
use glam::f32::*;
use protocol::servermsg_generated::{ServerMessage, ServerMessageInner};
use specs::{Builder, Entity, EntityBuilder, Join, WorldExt};
pub mod player;
use player::{create_background, create_our_player, create_remote_player};

pub struct WorldManager {
  me: Option<Entity>,
  background: Option<Entity>,
  camera_y: f32,
  /// only used for tracking server bells
  entityid_map: HashMap<EntityId, Entity>,
  local_bell_gen: Option<BellGenContext>,
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
      background: None,
      camera_y: CAMERA_INIT_Y,
      entityid_map: HashMap::new(),
      local_bell_gen: None,
      state: GameState::Connecting,
    }
  }

  pub fn init_common(&mut self, ec: &mut EcCtx) {
    if self.background.is_none() {
      self.background = Some(create_background(ec));
    }
  }

  pub fn init_offline(&mut self, ec: &mut EcCtx) {
    self.init_common(ec);
    if self.me.is_none() {
      self.me = Some(create_our_player(ec));
    } // otherwise we can just keep the current player position and "switch seamlessly" from online to offline.
    let mut bell_gen = BellGenContext::new();
    let mut highest_pos: Option<Vec2> = None;
    for (_, bell_pos) in (
      &ec.world.read_storage::<BellComponent>(),
      &ec.world.read_storage::<WorldSpaceTransform>(),
    )
      .join()
    {
      let p = bell_pos.position();
      let this_pos = Vec2::new(p.x, p.y);
      if highest_pos.is_none() || highest_pos.unwrap().y < this_pos.y {
        highest_pos = Some(this_pos);
      }
    }
    if let Some(pos) = highest_pos {
      bell_gen.set_last_point(pos);
    }
    ec.world.maintain();
    self.local_bell_gen = Some(bell_gen);
    self.state = GameState::Offline;
  }

  pub fn show_connection_error(&mut self, _ec: &mut EcCtx) {
    // todo
  }

  pub fn init_online(&mut self, ec: &mut EcCtx) {
    ec.world.delete_all();
    self.background = None;
    self.me = None;
    self.local_bell_gen = None;
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

  fn attach_bell_client_commponent(ent: EntityBuilder) -> EntityBuilder {
    ent
      .with(DrawImage {
        texture: &global::get_ref().graphics.images.gopher,
        size: Vec2::new(1f32, 1f32),
        alpha: 1f32,
      })
      .with(OurJumpableBell)
  }

  pub fn offline_update(&mut self, ec: &mut EcCtx) {
    let player_pos = ec
      .world
      .read_storage::<WorldSpaceTransform>()
      .get(self.me.unwrap())
      .map(|x| x.position());
    if let Some(player_pos) = player_pos {
      let bell_gen = self.local_bell_gen.as_mut().unwrap();
      bell_gen.ensure(
        player_pos.y + 12f32,
        &mut ec.world,
        Self::attach_bell_client_commponent,
      )
    }
  }

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
      ServerMessageInner::NONE => unreachable!(),
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
        let _msg = msg.msg_as_player_delete().unwrap();
      }
    }
  }
}

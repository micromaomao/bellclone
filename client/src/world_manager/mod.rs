use std::collections::HashMap;

use crate::{ec::{BlurFlags, components::{
    bell::OurJumpableBell,
    player::{OurPlayer, OurPlayerState},
    DrawImage,
  }}, global, render::{view::view_matrix, ViewportSize}};
use crate::{ec::EcCtx, render::view::ViewportInfo};
use game_core::{
  ec::{
    components::{physics::Velocity, transform::WorldSpaceTransform, EntityId},
    DeltaTime,
  },
  gen::BellGenContext,
  STAGE_MIN_HEIGHT, STAGE_WIDTH,
};
use glam::f32::*;
use specs::{Builder, Entity, EntityBuilder, WorldExt};
pub mod player;
use player::{create_background, create_player_local};

pub struct WorldManager {
  me: Option<Entity>,
  background: Option<Entity>,
  camera_y: f32,
  /// only used for tracking server bells
  bells: HashMap<EntityId, Entity>,
  local_bell_gen: Option<BellGenContext>,
}

pub const CAMERA_OFFSET_Y: f32 = -4f32;
pub const CAMERA_INIT_Y: f32 = -2f32;
pub const CAMERA_TARGET_EPSILON: f32 = 0.1f32;
pub const CAMERA_SPEED_MUL: f32 = 4f32;
pub const CAMERA_SWITCH_TO_GROUND_EARLY_PERIOD: f32 = 0.5f32; // secs

impl WorldManager {
  pub fn new(ec: &mut EcCtx) -> Self {
    ec.world.maintain();
    // todo
    WorldManager {
      me: None,
      background: None,
      camera_y: CAMERA_INIT_Y,
      bells: HashMap::new(),
      local_bell_gen: None,
    }
  }

  pub fn init_common(&mut self, ec: &mut EcCtx) {
    self.background = Some(create_background(ec));
  }

  pub fn init_offline(&mut self, ec: &mut EcCtx) {
    self.init_common(ec);
    self.me = Some(create_player_local(ec));
    self.local_bell_gen = Some(BellGenContext::new());
  }

  pub fn update(&mut self, ec: &mut EcCtx) {
    self.offline_update(ec);
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
}

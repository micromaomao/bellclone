use std::collections::HashMap;

use crate::{ec::components::debug::DebugRect, render::view::view_matrix};
use crate::{ec::EcCtx, render::view::ViewportInfo};
use game_core::{
  ec::{
    components::{physics::Velocity, transform::WorldSpaceTransform, EntityId},
    DeltaTime,
  },
  gen::BellGenContext,
};
use glam::f32::*;
use specs::{Builder, Entity, WorldExt};
pub mod player;
use player::{create_background, create_player_local};

pub struct WorldManager {
  me: Entity,
  background: Entity,
  camera_y: f32,
  /// only used for tracking server bells
  bells: HashMap<EntityId, Entity>,
  local_bell_gen: Option<BellGenContext>,
}

pub const CAMERA_OFFSET: f32 = -3f32;
pub const CAMERA_TARGET_EPSILON: f32 = 0.1f32;
pub const CAMERA_MAX_SPEED: f32 = 50f32;
pub const CAMERA_SPEED_MUL: f32 = 2f32;

impl WorldManager {
  pub fn new(ec: &mut EcCtx) -> Self {
    let me = create_player_local(ec);
    let background = create_background(ec);
    ec.world.maintain();
    // todo
    WorldManager {
      me,
      background,
      camera_y: CAMERA_OFFSET,
      bells: HashMap::new(),
      local_bell_gen: None,
    }
  }

  pub fn init_offline(&mut self, c: &mut EcCtx) {
    self.local_bell_gen = Some(BellGenContext::new());
  }

  pub fn update(&mut self, ec: &mut EcCtx) {
    self.offline_update(ec);
  }

  pub fn offline_update(&mut self, ec: &mut EcCtx) {
    let player_pos = ec
      .world
      .read_storage::<WorldSpaceTransform>()
      .get(self.me)
      .map(|x| x.position());
    if let Some(player_pos) = player_pos {
      let bell_gen = self.local_bell_gen.as_mut().unwrap();
      bell_gen.ensure(player_pos.y + 12f32, &mut ec.world, |ent| {
        ent.with(DebugRect::with_size(1f32))
      })
    }
  }

  pub fn calculate_camera(&mut self, ec: &EcCtx, width: u32, height: u32) -> ViewportInfo {
    let player_y = ec
      .world
      .read_storage::<WorldSpaceTransform>()
      .get(self.me)
      .map(|x| x.position().y)
      .unwrap_or(0f32);
    let player_v = ec
      .world
      .read_storage::<Velocity>()
      .get(self.me)
      .map(|x| x.0.y)
      .unwrap_or(0f32);
    let mut cam_y = self.camera_y;
    let target_y = f32::max(player_y + CAMERA_OFFSET, CAMERA_OFFSET);
    let dt = ec.world.read_resource::<DeltaTime>().as_secs_f32();
    if target_y - cam_y > CAMERA_TARGET_EPSILON {
      cam_y += dt * f32::min(CAMERA_MAX_SPEED, CAMERA_SPEED_MUL * (target_y - cam_y));
    } else if cam_y - target_y > CAMERA_TARGET_EPSILON {
      cam_y -= dt * f32::min(CAMERA_MAX_SPEED, CAMERA_SPEED_MUL * (cam_y - target_y));
    }
    cam_y += (player_v * 0.5).min(CAMERA_MAX_SPEED) * dt;
    if player_y < 10f32 && player_v < -20f32 {
      cam_y = -2f32;
    }
    if cam_y < -2f32 {
      cam_y = -2f32;
    }
    self.camera_y = cam_y;
    view_matrix(width, height, cam_y)
  }
}

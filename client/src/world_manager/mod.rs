use crate::render::view::view_matrix;
use crate::{ec::EcCtx, render::view::ViewportInfo};
use game_core::ec::{components::transform::WorldSpaceTransform, DeltaTime};
use glam::f32::*;
use specs::{Entity, WorldExt};
pub mod player;
use player::{create_background, create_player_local};

pub struct WorldManager {
  me: Entity,
  background: Entity,
  camera_y: f32,
}

pub const CAMERA_OFFSET: f32 = -2f32;
pub const CAMERA_TARGET_EPSILON: f32 = 4f32;

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
    }
  }

  pub fn update(&mut self, ec: &mut EcCtx) {}

  pub fn view_matrix(&mut self, ec: &EcCtx, width: u32, height: u32) -> ViewportInfo {
    let player_y = ec
      .world
      .read_storage::<WorldSpaceTransform>()
      .get(self.me)
      .map(|x| x.position().y)
      .unwrap_or(0f32);
    let mut cam_y = self.camera_y;
    let mut target_y = f32::max(player_y + CAMERA_OFFSET, CAMERA_OFFSET);
    if player_y < 6f32 {
      target_y = CAMERA_OFFSET;
    }
    let dt = ec.world.read_resource::<DeltaTime>().as_secs_f32();
    if target_y - cam_y > CAMERA_TARGET_EPSILON {
      cam_y += dt * f32::min(10f32, 5f32 * (target_y - cam_y));
    } else if cam_y - target_y > CAMERA_TARGET_EPSILON {
      cam_y -= dt * f32::min(10f32, 5f32 * (cam_y - target_y));
    }
    self.camera_y = cam_y;
    view_matrix(width, height, cam_y)
  }
}

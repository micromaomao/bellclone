use crate::ec::{
  components::{debug::DebugRect, player::OurPlayer},
  EcCtx,
};
use crate::render::view::view_matrix;
use game_core::ec::{
  components::{player::PlayerComponent, transform::WorldSpaceTransform},
  DeltaTime,
};
use glam::f32::*;
use specs::{Builder, Entity, WorldExt};
use uuid::Uuid;
pub struct WorldManager {
  me: Entity,
  background: Entity,
  camera_y: f32,
}

pub const CAMERA_OFFSET: f32 = -2f32;
pub const CAMERA_TARGET_EPSILON: f32 = 2f32;

impl WorldManager {
  pub fn new(ec: &mut EcCtx) -> Self {
    let me = Self::create_our_player(ec);
    let background = Self::create_background(ec);
    ec.world.maintain();
    // todo
    WorldManager {
      me,
      background,
      camera_y: CAMERA_OFFSET,
    }
  }

  fn create_our_player(ec: &mut EcCtx) -> Entity {
    ec.world
      .create_entity()
      .with(OurPlayer {})
      .with(PlayerComponent {
        id: Uuid::default(),
      })
      .with(WorldSpaceTransform::from_pos(0.5f32 * Vec3::unit_y()))
      .with(DebugRect::with_size(0.2f32))
      .build()
  }

  fn create_background(ec: &mut EcCtx) -> Entity {
    ec.world
      .create_entity()
      .with(WorldSpaceTransform::from_pos(Vec3::zero()))
      .with(DebugRect::default())
      .build()
  }

  pub fn update(&mut self, ec: &mut EcCtx) {}

  pub fn view_matrix(&mut self, ec: &EcCtx, aspect_ratio: f32) -> Mat4 {
    let target_y = f32::max(
      ec.world
        .read_storage::<WorldSpaceTransform>()
        .get(self.me)
        .map(|x| x.position().y)
        .unwrap_or(0f32)
        + CAMERA_OFFSET,
      CAMERA_OFFSET,
    );
    let dt = ec.world.read_resource::<DeltaTime>().as_secs_f32();
    let mut cam_y = self.camera_y;
    if target_y - cam_y > CAMERA_TARGET_EPSILON {
      cam_y += dt * f32::min(10f32, 5f32 * (target_y - cam_y));
    } else if cam_y - target_y > CAMERA_TARGET_EPSILON {
      cam_y -= dt * f32::min(10f32, 5f32 * (cam_y - target_y));
    }
    self.camera_y = cam_y;
    view_matrix(aspect_ratio, cam_y)
  }
}

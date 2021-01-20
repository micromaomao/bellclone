use crate::ec::{components::debug::DebugRect, EcCtx};
use game_core::ec::components::transform::WorldSpaceTransform;
use glam::f32::*;
use specs::{Builder, WorldExt};
pub struct WorldManager;

impl WorldManager {
  pub fn new(ec: &mut EcCtx) -> Self {
    let ent = ec
      .world
      .create_entity()
      .with(WorldSpaceTransform::from_pos(Vec3::zero()))
      .with(DebugRect::default())
      .build();
    ec.world.maintain();
    // todo
    WorldManager
  }

  pub fn update(&mut self, ec: &mut EcCtx) {}
}

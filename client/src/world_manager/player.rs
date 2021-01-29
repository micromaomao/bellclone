use crate::ec::{
  components::{debug::DebugRect, player::OurPlayer},
  EcCtx,
};
use game_core::ec::components::{
  physics::{Gravity, Velocity},
  player::PlayerComponent,
  transform::WorldSpaceTransform,
  EntityId,
};
use glam::f32::*;
use specs::{Builder, Entity, WorldExt};

pub fn create_player_local(ec: &mut EcCtx) -> Entity {
  ec.world
    .create_entity()
    .with(OurPlayer {})
    .with(PlayerComponent {})
    .with(EntityId::new())
    .with(WorldSpaceTransform::from_pos(0.5f32 * Vec3::unit_y()))
    .with(Velocity::default())
    .with(Gravity::default())
    .with(DebugRect::with_size(0.2f32))
    .build()
}

pub fn create_background(ec: &mut EcCtx) -> Entity {
  ec.world
    .create_entity()
    .with(WorldSpaceTransform::from_pos(Vec3::zero()))
    .with(DebugRect::default())
    .build()
}

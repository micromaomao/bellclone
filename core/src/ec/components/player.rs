use crate::ec::components::{
  physics::{Gravity, Velocity},
  transform::WorldSpaceTransform,
  EntityId,
};
use glam::f32::*;
use specs::{Builder, Component, DenseVecStorage, EntityBuilder, World, WorldExt};

pub struct PlayerComponent {
  pub score: u128,
}

impl Component for PlayerComponent {
  type Storage = DenseVecStorage<Self>;
}

pub fn build_player(world: &mut World) -> EntityBuilder {
  world
    .create_entity()
    .with(PlayerComponent { score: 0u128 })
    .with(EntityId::new())
    .with(WorldSpaceTransform::from_pos(Vec3::zero()))
    .with(Velocity::default())
    .with(Gravity::default())
}

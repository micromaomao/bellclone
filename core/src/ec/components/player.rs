use glam::f32::*;
use specs::{Builder, Component, DenseVecStorage, EntityBuilder, World, WorldExt};
use crate::ec::components::{EntityId, physics::{Gravity, Velocity}, transform::WorldSpaceTransform};

pub struct PlayerComponent {
}

impl Component for PlayerComponent {
  type Storage = DenseVecStorage<Self>;
}

pub fn build_player(world: &mut World) -> EntityBuilder {
  world
    .create_entity()
    .with(PlayerComponent {})
    .with(EntityId::new())
    .with(WorldSpaceTransform::from_pos(Vec3::zero()))
    .with(Velocity::default())
    .with(Gravity::default())
}

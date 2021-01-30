use glam::f32::*;
use specs::{Builder, Component, EntityBuilder, VecStorage, World, WorldExt};

use super::transform::WorldSpaceTransform;

pub struct BellComponent {
  pub size: f32,
}

impl Component for BellComponent {
  type Storage = VecStorage<BellComponent>;
}

pub fn build_bell(world: &mut World, size: f32, point: Vec2) -> EntityBuilder {
  world.create_entity().with(BellComponent { size }).with(
    WorldSpaceTransform::from_pos(Vec3::new(point.x, point.y, 0f32))
      .add(Mat4::from_scale(Vec3::new(size, size, 1f32))),
  )
}

use specs::{Component, DenseVecStorage};
use glam::f32::*;

#[derive(Default)]
pub struct CollisionStar {
  pub alive_time: f32,
  pub base_transform: Mat4,
}

impl Component for CollisionStar {
  type Storage = DenseVecStorage<CollisionStar>;
}

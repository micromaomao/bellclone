use glam::f32::*;
use specs::{Component, DenseVecStorage};

#[derive(Default)]
pub struct CollisionStar {
  pub alive_time: f32,
  pub base_transform: Mat4,
}

impl Component for CollisionStar {
  type Storage = DenseVecStorage<CollisionStar>;
}

use glam::f32::*;
use specs::{Component, DenseVecStorage};

pub struct PlayerComponent {
}

impl Component for PlayerComponent {
  type Storage = DenseVecStorage<Self>;
}

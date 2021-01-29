use glam::f32::*;
use specs::{Component, NullStorage, VecStorage};

#[derive(Debug, Default)]
pub struct Velocity(pub Vec2);

impl Component for Velocity {
  type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Gravity;

impl Component for Gravity {
  type Storage = NullStorage<Self>;
}

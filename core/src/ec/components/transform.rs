use glam::f32::*;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct WorldSpaceTransform(pub Mat4);
impl Component for WorldSpaceTransform {
  type Storage = VecStorage<Self>;
}

impl WorldSpaceTransform {
  pub fn position(&self) -> Vec3 {
    self.0.transform_point3(Vec3::zero())
  }
  pub fn add(&self, transform: Mat4) -> Self {
    Self(self.0 * transform)
  }
  pub fn from_pos(pos: Vec3) -> Self {
    WorldSpaceTransform(Mat4::from_translation(pos))
  }
}

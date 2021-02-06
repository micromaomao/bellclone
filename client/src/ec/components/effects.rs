use specs::{Component, DenseVecStorage};

#[derive(Debug, Clone, Copy)]
pub struct FadeOut {
  pub alive: f32,
  pub total_time: f32,
}

impl FadeOut {
  pub fn new(life: f32) -> Self {
    Self {
      alive: 0f32,
      total_time: life
    }
  }
}

impl Component for FadeOut {
  type Storage = DenseVecStorage<Self>;
}

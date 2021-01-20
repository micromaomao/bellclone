use specs::{Component, DenseVecStorage};

#[derive(Debug)]
pub enum DebugRectStyle {
  CrossedBox,
}
#[derive(Debug)]
pub struct DebugRect {
  pub style: DebugRectStyle,
  pub width: f32,
  pub height: f32,
}

impl Component for DebugRect {
  type Storage = DenseVecStorage<Self>;
}

impl Default for DebugRect {
  fn default() -> Self {
    DebugRect {
      style: DebugRectStyle::CrossedBox,
      width: 1f32,
      height: 1f32,
    }
  }
}

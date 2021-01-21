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

impl DebugRect {
  pub fn with_size(size: f32) -> Self {
    let mut d = Self::default();
    d.width = size;
    d.height = size;
    d
  }
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

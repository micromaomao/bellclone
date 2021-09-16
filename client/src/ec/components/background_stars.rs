use std::ops::Range;

use specs::{Component, VecStorage};

pub struct BackgroundStar {
  pub alpha_range: Range<f32>,
  pub current_alpha_increasing: bool,
}

impl Component for BackgroundStar {
  type Storage = VecStorage<BackgroundStar>;
}

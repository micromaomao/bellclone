use specs::{Component, VecStorage};

pub struct BellComponent {
  pub size: f32,
}

impl Component for BellComponent {
  type Storage = VecStorage<BellComponent>;
}

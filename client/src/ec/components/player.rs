use specs::{Component, DenseVecStorage};

pub struct OurPlayer {}

impl Component for OurPlayer {
  type Storage = DenseVecStorage<Self>;
}

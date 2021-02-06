use specs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct OurPlayer {
  pub state: OurPlayerState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OurPlayerState {
  NotStarted,
  Flying,
  Falling,
}

impl Component for OurPlayer {
  type Storage = DenseVecStorage<Self>;
}

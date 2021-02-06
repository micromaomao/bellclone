use specs::{Component, DenseVecStorage, Entity};

#[derive(Debug)]
pub struct OurPlayer {
  pub state: OurPlayerState,
  pub next_bell_score: u128,
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

#[derive(Debug, Clone, Copy)]
pub struct WithScoreDisplay(pub Entity);

impl Component for WithScoreDisplay {
  type Storage = DenseVecStorage<Self>;
}

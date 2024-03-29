use std::str::FromStr;

use specs::{Component, DenseVecStorage};
use uuid::Uuid;

pub mod bell;
pub mod physics;
pub mod player;
pub mod transform;
pub mod bird;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct EntityId(pub Uuid);

impl Component for EntityId {
  type Storage = DenseVecStorage<Self>;
}

impl EntityId {
  pub fn new() -> Self {
    EntityId(Uuid::new_v4())
  }
}

impl FromStr for EntityId {
  type Err = <Uuid as FromStr>::Err;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(Uuid::from_str(s)?))
  }
}

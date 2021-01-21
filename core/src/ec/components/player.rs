use specs::{Component, DenseVecStorage};
use uuid::Uuid;

pub struct PlayerComponent {
  pub id: Uuid
}

impl Component for PlayerComponent {
  type Storage = DenseVecStorage<Self>;
}

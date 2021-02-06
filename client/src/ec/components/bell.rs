use specs::{Component, NullStorage};

#[derive(Default)]
pub struct OurJumpableBell;

impl Component for OurJumpableBell {
  type Storage = NullStorage<Self>;
}

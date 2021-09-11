use specs::{Component, VecStorage};

pub struct BellSequenceNumber(pub u64);
impl Component for BellSequenceNumber {
  type Storage = VecStorage<BellSequenceNumber>;
}

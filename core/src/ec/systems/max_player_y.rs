use specs::{ReadStorage, System, Write, Join};

use std::borrow::BorrowMut;

use crate::ec::components::{player::PlayerComponent, transform::WorldSpaceTransform};

pub struct MaxPlayerYSystem;
#[derive(Default)]
pub struct MaxPlayerY(pub Option<f32>);

impl<'a> System<'a> for MaxPlayerYSystem {
  type SystemData = (
    ReadStorage<'a, PlayerComponent>,
    ReadStorage<'a, WorldSpaceTransform>,
    Write<'a, MaxPlayerY>,
  );

  fn run(&mut self, (playerc, wstc, mut res): Self::SystemData) {
    let max_player_y = (&playerc, &wstc)
      .join()
      .map(|(_, w)| w.position().y)
      .reduce(f32::max);
    res.0 = max_player_y;
  }
}

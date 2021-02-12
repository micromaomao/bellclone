use game_core::ec::components::bell::BellComponent;
use specs::{Join, ReadStorage, System, WriteStorage};

use crate::ec::components::{bell::OurJumpableBell, DrawImage};

pub struct BellSystem;

impl<'a> System<'a> for BellSystem {
  type SystemData = (
    ReadStorage<'a, BellComponent>,
    ReadStorage<'a, OurJumpableBell>,
    WriteStorage<'a, DrawImage>,
  );

  fn run(&mut self, (bells, our_jumpable_bells, mut draw_images): Self::SystemData) {
    for (_, _, dimg) in (&bells, !&our_jumpable_bells, &mut draw_images).join() {
      dimg.alpha = 0.3f32;
    }
    for (_, _, dimg) in (&bells, &our_jumpable_bells, &mut draw_images).join() {
      dimg.alpha = 1f32;
    }
  }
}

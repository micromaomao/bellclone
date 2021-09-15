use game_core::ec::components::{bell::BellComponent, transform::WorldSpaceTransform};
use glam::Vec2;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::ec::components::{DrawImage, bell::OurJumpableBell, effects::FadeOut};

pub struct BellSystem;

impl<'a> System<'a> for BellSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, BellComponent>,
    WriteStorage<'a, OurJumpableBell>,
    WriteStorage<'a, DrawImage>,
    WriteStorage<'a, FadeOut>,
    ReadStorage<'a, WorldSpaceTransform>,
  );

  fn run(&mut self, (ents, bells, mut our_jumpable_bells, mut draw_images, mut fadeoutc, trc): Self::SystemData) {
    for (ent, bell_c, tr) in (&ents, &bells, &trc).join() {
      if !draw_images.contains(ent) {
        draw_images.insert(ent, DrawImage {
          texture: &crate::global::get_ref().graphics.images.gopher,
          size: Vec2::new(1f32, 1f32),
          alpha: 1f32,
        });
        our_jumpable_bells.insert(ent, OurJumpableBell);
      }
      if tr.position().y < 1f32 && !fadeoutc.contains(ent) {
        fadeoutc.insert(ent, FadeOut::new(1f32));
      }
    }
  }
}

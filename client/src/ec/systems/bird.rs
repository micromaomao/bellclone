use game_core::ec::components::bird::Bird;
use glam::Vec2;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::{ec::components::{DrawImage, bell::OurJumpableBell}};

pub struct ClientBirdSystem;

impl<'a> System<'a> for ClientBirdSystem {
  type SystemData = (
    ReadStorage<'a, Bird>,
    Entities<'a>,
    WriteStorage<'a, DrawImage>,
    WriteStorage<'a, OurJumpableBell>,
  );

  fn run(&mut self, (birdc, ents, mut drawc, mut jumpc): Self::SystemData) {
    for (ent, _) in (&ents, &birdc).join() {
      if !drawc.contains(ent) {
        drawc.insert(ent, DrawImage {
          texture: &crate::global::get_ref().graphics.images.cargo,
          alpha: 1f32,
          size: Vec2::new(0.5f32, 0.5f32)
        });
        jumpc.insert(ent, OurJumpableBell);
      }
    }
  }
}

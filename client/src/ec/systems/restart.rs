use game_core::ec::components::{bell::BellComponent, player::PlayerComponent};
use specs::{Entities, Entity, Join, ReadStorage, System, WriteStorage};

use crate::ec::components::{DrawImage, bell::OurJumpableBell, player::{OurPlayer, OurPlayerState}};

#[derive(Debug, Default)]
pub struct RestartSystem;

impl<'a> System<'a> for RestartSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, OurPlayer>,
    WriteStorage<'a, PlayerComponent>,
    ReadStorage<'a, BellComponent>,
    WriteStorage<'a, OurJumpableBell>,
    WriteStorage<'a, DrawImage>,
  );

  fn run(&mut self, (ents, mut ops, mut ps, bells, mut ojbs, mut dics): Self::SystemData) {
    for (op, p) in (&mut ops, &mut ps).join() {
      if op.state == OurPlayerState::NotStarted {
        for (ent, _, draw) in (&ents, &bells, &mut dics).join() {
          if !ojbs.contains(ent) {
            ojbs.insert(ent, OurJumpableBell);
            draw.alpha = 1f32;
          }
        }
        p.score = 0;
        op.next_bell_score = 10;
        break;
      }
    }
  }
}

use game_core::ec::components::{bell::BellComponent, bird::Bird, player::PlayerComponent};
use specs::{Entities, Entity, Join, ReadStorage, System, WriteStorage};

use crate::ec::components::{
  bell::OurJumpableBell,
  player::{OurPlayer, OurPlayerState},
  DrawImage,
};

#[derive(Debug, Default)]
pub struct RestartSystem;

impl<'a> System<'a> for RestartSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, OurPlayer>,
    WriteStorage<'a, PlayerComponent>,
    WriteStorage<'a, OurJumpableBell>,
    WriteStorage<'a, DrawImage>,
    ReadStorage<'a, BellComponent>,
    ReadStorage<'a, Bird>,
  );

  fn run(&mut self, (ents, mut ops, mut ps, mut jumpc, mut dics, bellc, birdc): Self::SystemData) {
    for (op, p) in (&mut ops, &mut ps).join() {
      if op.state == OurPlayerState::NotStarted {
        for (ent, draw) in (&ents, &mut dics).join() {
          if !jumpc.contains(ent) && (bellc.contains(ent) || birdc.contains(ent)) {
            jumpc.insert(ent, OurJumpableBell);
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

use game_core::ec::components::{bell::BellComponent, player::PlayerComponent};
use specs::{Entities, Entity, Join, ReadStorage, System, WriteStorage};

use crate::ec::components::{
  bell::OurJumpableBell,
  player::{OurPlayer, OurPlayerState},
};

#[derive(Debug, Default)]
pub struct RestartSystem {
  entbuf: Vec<Entity>,
}

impl<'a> System<'a> for RestartSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, OurPlayer>,
    WriteStorage<'a, PlayerComponent>,
    ReadStorage<'a, BellComponent>,
    WriteStorage<'a, OurJumpableBell>,
  );

  fn run(&mut self, (ents, mut ops, mut ps, bells, mut ojbs): Self::SystemData) {
    for (op, p) in (&mut ops, &mut ps).join() {
      if op.state == OurPlayerState::NotStarted {
        for (entid, _, _) in (&ents, &bells, !&ojbs).join() {
          self.entbuf.push(entid);
        }
        for &ent in self.entbuf.iter() {
          ojbs.insert(ent, OurJumpableBell).unwrap();
        }
        self.entbuf.clear();
        p.score = 0;
        op.next_bell_score = 10;
        break;
      }
    }
  }
}

use game_core::ec::components::{bell::BellComponent, transform::WorldSpaceTransform};
use specs::{Entities, Join, ReadStorage, System};

pub struct ServerBellsSystem;

impl<'a> System<'a> for ServerBellsSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, BellComponent>,
    ReadStorage<'a, WorldSpaceTransform>,
  );

  fn run(&mut self, (mut ents, mut bellc, mut wstc): Self::SystemData) {
    for (ent, _, w) in (&ents, &bellc, &wstc).join() {
      if w.position().y < 1f32 {
        ents.delete(ent);
      }
    }
  }
}

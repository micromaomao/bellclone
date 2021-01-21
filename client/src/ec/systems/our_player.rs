use game_core::ec::{
  components::{player::PlayerComponent, transform::WorldSpaceTransform},
  DeltaTime,
};
use glam::f32::*;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::ec::components::player::OurPlayer;

pub struct OurPlayerSystem;

impl<'a> System<'a> for OurPlayerSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    WriteStorage<'a, PlayerComponent>,
    ReadStorage<'a, OurPlayer>,
    WriteStorage<'a, WorldSpaceTransform>,
  );

  fn run(&mut self, (dt, mut players, our_players, mut trs): Self::SystemData) {
    let dt = dt.as_secs_f32();
    for (p, our_p, tr) in (&mut players, &our_players, &mut trs).join() {
      tr.add_to_self(Mat4::from_translation(Vec3::new(0f32, -1f32 * dt, 0f32)));
      if tr.position().y < 0f32 {
        tr.add_to_self(Mat4::from_translation(Vec3::new(
          0f32,
          -tr.position().y,
          0f32,
        )));
      }
    }
  }
}

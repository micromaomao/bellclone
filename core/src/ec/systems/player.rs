use specs::{Join, ReadStorage, System, WriteStorage};

use crate::ec::components::{
  physics::Velocity, player::PlayerComponent, transform::WorldSpaceTransform,
};
use glam::f32::*;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
  type SystemData = (
    ReadStorage<'a, PlayerComponent>,
    WriteStorage<'a, WorldSpaceTransform>,
    WriteStorage<'a, Velocity>,
  );

  fn run(&mut self, (ps, mut trs, mut vels): Self::SystemData) {
    for (_, transform, vel) in (&ps, &mut trs, &mut vels).join() {
      let pos = transform.position();
      if pos.y < -0.001f32 {
        transform.add_to_self(Mat4::from_translation(Vec3::new(0f32, -pos.y, 0f32)));
        vel.0.y = 0f32;
      }
    }
  }
}

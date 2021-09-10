use crate::ec::{
  components::{physics::Velocity, transform::WorldSpaceTransform},
  DeltaTime,
};
use glam::f32::*;
use specs::{Join, Read};
use specs::{ReadStorage, System, WriteStorage};

pub struct VelocitySystem;

impl<'a> System<'a> for VelocitySystem {
  type SystemData = (
    WriteStorage<'a, WorldSpaceTransform>,
    ReadStorage<'a, Velocity>,
    Read<'a, DeltaTime>,
  );

  fn run(&mut self, (mut trs, vels, dt): Self::SystemData) {
    let dt = dt.as_secs_f32();
    for (tr, vel) in (&mut trs, &vels).join() {
      let vel = &vel.0;
      tr.add_to_self(Mat4::from_translation(Vec3::new(vel.x, vel.y, 0f32) * dt));
    }
  }
}

pub struct GravitySystem;
pub const GRAVITY: f32 = 26f32;
pub const TERMINAL: f32 = 200f32;

impl<'a> System<'a> for GravitySystem {
  type SystemData = (WriteStorage<'a, Velocity>, Read<'a, DeltaTime>);

  fn run(&mut self, (mut vels, dt): Self::SystemData) {
    let dt = dt.as_secs_f32();
    let dv = Vec2::new(0f32, -GRAVITY) * dt;
    for vel in (&mut vels).join() {
      vel.0 += dv;
      if vel.0.y < -TERMINAL {
        vel.0.y = -TERMINAL;
      }
    }
  }
}

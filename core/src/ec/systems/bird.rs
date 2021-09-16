use crate::{STAGE_MAX_X, STAGE_MIN_X, ec::{DeltaTime, components::{bird::Bird, bird::Direction, physics::Velocity, transform::WorldSpaceTransform}}};
use glam::{Mat4, Vec2, Vec3};
use specs::{Entities, Join, Read, System, WriteStorage};

pub struct BirdSystem;

pub const MOVE_SPEED: f32 = 2f32;

impl<'a> System<'a> for BirdSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, Bird>,
    WriteStorage<'a, WorldSpaceTransform>,
    WriteStorage<'a, Velocity>,
    Read<'a, DeltaTime>,
  );

  fn run(&mut self, (ent, mut birdc, mut wtc, mut velc, delta_time): Self::SystemData) {
    for (ent, bird, wst) in (&ent, &mut birdc, &mut wtc).join() {
      if bird.turning {
        if bird.direction.turn_to_opposite_animation(*delta_time, wst) {
          bird.turning = false;
          bird.direction = bird.direction.reverse();
        }
      } else {
        match bird.direction {
          Direction::LEFT => {
            if wst.position().x < STAGE_MIN_X + 1f32 {
              bird.turning = true;
              velc.remove(ent);
            } else {
              // velocity is in local coordinate
              velc.insert(ent, Velocity(Vec2::X * MOVE_SPEED));
            }
          },
          Direction::RIGHT => {
            if wst.position().x > STAGE_MAX_X - 1f32 {
              bird.turning = true;
              velc.remove(ent);
            } else {
              // velocity is in local coordinate
              velc.insert(ent, Velocity(Vec2::X * MOVE_SPEED));
            }
          }
        }
      }
    }
  }
}

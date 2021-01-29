use game_core::ec::{
  components::{physics::Velocity, player::PlayerComponent, transform::WorldSpaceTransform},
  DeltaTime,
};
use glam::f32::*;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::ec::{components::player::OurPlayer, user_input::PointerState};

pub struct OurPlayerSystem;

pub const X_VELOCITY_UPPER_BOUND: f32 = 25f32;
pub const X_VELOCITY_LOWER_BOUND: f32 = 2f32;
pub const POINTER_CHASE_SPEED: f32 = 5f32;
pub const JUMP_VELOCITY: f32 = 14f32;

impl<'a> System<'a> for OurPlayerSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Read<'a, PointerState>,
    WriteStorage<'a, PlayerComponent>,
    ReadStorage<'a, OurPlayer>,
    ReadStorage<'a, WorldSpaceTransform>,
    WriteStorage<'a, Velocity>,
  );

  fn run(&mut self, (dt, ps, mut players, our_players, trs, mut vels): Self::SystemData) {
    let dt = dt.as_secs_f32();
    for (p, our_p, tr, vel) in (&mut players, &our_players, &trs, &mut vels).join() {
      let pos = tr.position();

      if let Some(pointer_ws) = ps.world_space {
        let target_x = pointer_ws.x;
        let diff = target_x - pos.x;
        if diff.abs() > 0.01f32 {
          let mut xspeed = diff * POINTER_CHASE_SPEED;
          if xspeed.abs() > X_VELOCITY_UPPER_BOUND {
            xspeed = xspeed.signum() * X_VELOCITY_UPPER_BOUND;
          }
          if xspeed.abs() < X_VELOCITY_LOWER_BOUND {
            xspeed = xspeed.signum() * X_VELOCITY_LOWER_BOUND;
          }
          vel.0.x = xspeed;
        } else {
          vel.0.x = 0f32;
        }
      }

      if pos.y < 0.01f32 {
        vel.0.y = JUMP_VELOCITY;
      }
    }
  }
}

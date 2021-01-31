use game_core::ec::{
  components::{
    bell::BellComponent, physics::Velocity, player::PlayerComponent, transform::WorldSpaceTransform,
  },
  DeltaTime,
};
use glam::f32::*;
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

use crate::ec::{
  components::player::{OurPlayer, OurPlayerState},
  user_input::PointerState,
};

pub struct OurPlayerSystem;

pub const X_SPEED_UPPER_BOUND: f32 = 40f32;
pub const X_SPEED_LOWER_BOUND: f32 = 2f32;
pub const POINTER_CHASE_SPEED: f32 = 5f32;
pub const JUMP_SPEED: f32 = 14f32;
pub const WALK_SPEED: f32 = 6f32;
pub const JUMP_BELL_SPEED: f32 = 14f32;
pub const JUMP_BELL_SPEED_CAP: f32 = 16f32;
pub const FALLING_THRESHOLD_SPEED: f32 = 20f32;

impl<'a> System<'a> for OurPlayerSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Read<'a, PointerState>,
    Entities<'a>,
    WriteStorage<'a, PlayerComponent>,
    WriteStorage<'a, OurPlayer>,
    ReadStorage<'a, WorldSpaceTransform>,
    WriteStorage<'a, Velocity>,
    ReadStorage<'a, BellComponent>,
  );

  fn run(
    &mut self,
    (dt, ps, ents, mut players, mut our_players, trs, mut vels, bells): Self::SystemData,
  ) {
    let dt = dt.as_secs_f32();
    for (p, mut our_p, tr, vel) in (&mut players, &mut our_players, &trs, &mut vels).join() {
      let player_pos = tr.position();
      if player_pos.y < 0.01f32 {
        our_p.state = OurPlayerState::NotStarted;
        if ps.pressing {
          vel.0.y = JUMP_SPEED;
        }
        if let Some(pointer_ws) = ps.world_space {
          let target_x = pointer_ws.x;
          let diff = target_x - player_pos.x;
          if diff.abs() > 0.1f32 {
            vel.0.x = diff.signum() * WALK_SPEED;
          } else {
            vel.0.x = 0f32;
          }
        }
      } else {
        if let Some(pointer_ws) = ps.world_space {
          let target_x = pointer_ws.x;
          let diff = target_x - player_pos.x;
          if diff.abs() > 0.01f32 {
            let mut xspeed = diff * POINTER_CHASE_SPEED;
            if xspeed.abs() > X_SPEED_UPPER_BOUND {
              xspeed = xspeed.signum() * X_SPEED_UPPER_BOUND;
            }
            if xspeed.abs() < X_SPEED_LOWER_BOUND {
              xspeed = xspeed.signum() * X_SPEED_LOWER_BOUND;
            }
            vel.0.x = xspeed;
          } else {
            vel.0.x = 0f32;
          }
        }

        if our_p.state != OurPlayerState::Falling {
          let mut jumped = false;
          for (ent, bell, tr) in (&ents, &bells, &trs).join() {
            let pos = tr.position();
            let size = bell.size;
            if (player_pos - pos).length_squared() < size {
              let mut v = vel.0.y;
              if v < 0f32 {
                v = 0f32;
              }
              v += JUMP_BELL_SPEED;
              if v > JUMP_BELL_SPEED_CAP {
                v = JUMP_BELL_SPEED_CAP;
              }
              vel.0.y = v;
              our_p.state = OurPlayerState::Flying;
              let _ = ents.delete(ent); // todo
              jumped = true;
              break;
            }
          }
          if !jumped {
            if vel.0.y < -FALLING_THRESHOLD_SPEED {
              our_p.state = OurPlayerState::Falling;
            }
          }
        }
      }
    }
  }
}

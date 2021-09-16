use game_core::ec::{
  components::{
    bell::BellComponent, bird::Bird, physics::Velocity, player::PlayerComponent,
    transform::WorldSpaceTransform,
  },
  DeltaTime,
};
use glam::f32::*;
use specs::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage};

use crate::ec::{
  components::{
    bell::OurJumpableBell,
    collision_star::CollisionStar,
    draw_numbers::{Align, DrawNumbersComponent},
    effects::FadeOut,
    player::{OurPlayer, OurPlayerState},
    DrawImage,
  },
  user_input::PointerState,
};

use super::collision_star;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace=window)]
extern "C" {
  fn play_hit_audio();
}

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
    WriteStorage<'a, WorldSpaceTransform>,
    WriteStorage<'a, Velocity>,
    ReadStorage<'a, BellComponent>,
    WriteStorage<'a, OurJumpableBell>,
    WriteStorage<'a, CollisionStar>,
    WriteStorage<'a, DrawImage>,
    WriteStorage<'a, FadeOut>,
    WriteStorage<'a, DrawNumbersComponent>,
    ReadStorage<'a, Bird>,
  );

  fn run(
    &mut self,
    (
      _dt,
      ps,
      ents,
      mut players,
      mut our_players,
      mut trs,
      mut vels,
      bells,
      mut jumpable_bell_markers,
      mut colstars,
      mut draw_images,
      mut fade_outs,
      mut dns,
      birdc,
    ): Self::SystemData,
  ) {
    for (p_entid, p, mut our_p, vel) in (&ents, &mut players, &mut our_players, &mut vels).join() {
      let tr = trs.get(p_entid).unwrap();
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
          let mut jumped_from: Option<Entity> = None;
          for (ent, bell, bird, tr, _) in (
            &ents,
            bells.maybe(),
            birdc.maybe(),
            &trs,
            &jumpable_bell_markers,
          )
            .join()
          {
            let pos = tr.position();
            let size = bell.map(|x| x.size).unwrap_or(0.5f32);
            if (player_pos - pos).length_squared() < size * 0.5f32 {
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
              jumped_from = Some(ent);
              break;
            }
          }
          if let Some(ent) = jumped_from {
            jumpable_bell_markers.remove(ent);
            play_hit_audio();
            draw_images.get_mut(ent).unwrap().alpha = 0.3f32;
            let pos = trs.get(ent).unwrap().position();
            collision_star::build_stars((&ents, &mut colstars, &mut draw_images, &mut trs), pos);
            let mut dn = DrawNumbersComponent::new(1.0f32, Align::Center);
            if bells.contains(ent) {
              dn.set_number(our_p.next_bell_score);
              p.score += our_p.next_bell_score;
              our_p.next_bell_score += 10;
            } else {
              dn.set_number(p.score);
              p.score *= 2;
            }
            let sg = ents.create();
            fade_outs.insert(sg, FadeOut::new(0.6f32)).unwrap();
            dns.insert(sg, dn).unwrap();
            trs
              .insert(
                sg,
                WorldSpaceTransform::from_pos(pos + Vec3::Y * 0.2f32)
                  .add(Mat4::from_scale(Vec3::new(0.2f32, 0.2f32, 1f32))),
              )
              .unwrap();
          } else if vel.0.y < -FALLING_THRESHOLD_SPEED {
            our_p.state = OurPlayerState::Falling;
          }
        }
      }
    }
  }
}

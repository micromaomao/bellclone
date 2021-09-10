use std::f32::consts::PI;

use game_core::ec::{components::transform::WorldSpaceTransform, DeltaTime};
use glam::f32::*;
use specs::{Entities, Join, Read, ReadStorage, System, World, WriteStorage};

use crate::{
  ec::components::{collision_star::CollisionStar, DrawImage},
  global,
};

pub struct CollisionStarSystem;

const STAR_LIVE: f32 = 0.6f32;
const STAR_INIT_DISTANCE: f32 = 0.75f32;
const STAR_TRAVEL_DISTANCE: f32 = 1.5f32;
const STAR_SIZE: f32 = 0.2f32;

impl<'a> System<'a> for CollisionStarSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Entities<'a>,
    WriteStorage<'a, CollisionStar>,
    WriteStorage<'a, WorldSpaceTransform>,
    WriteStorage<'a, DrawImage>,
  );

  fn run(&mut self, (dt, ents, mut cstars, mut trs, mut draw_images): Self::SystemData) {
    let dt = dt.as_secs_f32();
    for (entid, star, tr, di) in (&ents, &mut cstars, &mut trs, &mut draw_images).join() {
      star.alive_time += dt;
      if star.alive_time > STAR_LIVE {
        ents.delete(entid).unwrap();
        break;
      }
      let dist = STAR_INIT_DISTANCE + STAR_TRAVEL_DISTANCE * (1f32 - (1f32 - (star.alive_time / STAR_LIVE)).powi(6));
      let new_tr = star.base_transform * Mat4::from_translation(Vec3::X * dist);
      tr.0 = new_tr;
      di.alpha = 1f32 - (star.alive_time / STAR_LIVE);
    }
  }
}

pub fn build_stars<'a>(
  (entities, col_stars, draw_images, trs): (
    &Entities<'a>,
    &mut WriteStorage<'a, CollisionStar>,
    &mut WriteStorage<'a, DrawImage>,
    &mut WriteStorage<'a, WorldSpaceTransform>,
  ),
  around_pos: Vec3,
) {
  let base_mat = Mat4::from_translation(around_pos);
  const NB_STARS: f32 = 10f32;
  for rot_angle in (0..(NB_STARS as u32)).map(|i| 2f32 * PI * i as f32 / NB_STARS) {
    let part_base_mat = base_mat * Mat4::from_rotation_z(rot_angle);
    let ent = entities.create();
    col_stars
      .insert(
        ent,
        CollisionStar {
          alive_time: 0f32,
          base_transform: part_base_mat,
        },
      )
      .unwrap();
    draw_images
      .insert(
        ent,
        DrawImage {
          texture: &global::get_ref().graphics.images.star,
          size: Vec2::new(1f32, 1f32) * STAR_SIZE,
          alpha: 1f32,
        },
      )
      .unwrap();
    trs.insert(ent, WorldSpaceTransform(part_base_mat)).unwrap();
  }
}

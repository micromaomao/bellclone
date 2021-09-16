use glam::{Mat4, Vec2, Vec3};
use rand_distr::StandardNormal;
use specs::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::Range;

use crate::ec::components::physics::Velocity;
use crate::ec::components::{
  bell::BellComponent, player::PlayerComponent, transform::WorldSpaceTransform,
};
use crate::{STAGE_MAX_X, STAGE_MIN_HEIGHT, STAGE_MIN_X};
use glam::Vec3Swizzles;
use rand::Rng;

#[derive(Default)]
pub struct CreateBellSystem {
  last_created: Option<Entity>,
}

pub const LOWEST_Y: f32 = 2f32;
pub const X_STDDEV: f32 = 3.5f32;
pub const Y_VARIATION: Range<f32> = 1.8f32..2.8f32;
pub const BELL_SIZE: f32 = 0.5f32;
pub const BELL_Y_VEL: f32 = 1f32;

#[derive(Default)]
pub struct CreateBellSystemControl {
  pub enabled: bool,
  pub last_round_gen: Vec<Entity>,
}

impl<'a> System<'a> for CreateBellSystem {
  type SystemData = (
    Write<'a, CreateBellSystemControl>,
    Entities<'a>,
    WriteStorage<'a, BellComponent>,
    WriteStorage<'a, WorldSpaceTransform>,
    ReadStorage<'a, PlayerComponent>,
    WriteStorage<'a, Velocity>,
  );

  fn run(&mut self, (mut ctl, ents, mut bellc, mut wst, mut playerc, mut velc): Self::SystemData) {
    if !ctl.enabled {
      return;
    }
    ctl.last_round_gen.clear();
    let max_player_y = (&playerc, &wst)
      .join()
      .map(|(_, w)| w.position().y)
      .reduce(f32::max);
    if max_player_y.is_none() {
      return;
    }
    if let Some(ent) = self.last_created {
      if !ents.is_alive(ent) {
        self.last_created = None;
      }
    }
    let max_player_y = max_player_y.unwrap();
    loop {
      let mut last_point = self
        .last_created
        .map(|e| wst.get(e).unwrap().position().xy());
      if last_point.map(|v| v.y).unwrap_or(-1f32) > max_player_y + STAGE_MIN_HEIGHT {
        break;
      }
      if last_point.is_none() {
        last_point = (&bellc, &wst).join().reduce(|a, b| {
          if a.1.position().y > b.1.position().y { a } else { b }
        }).map(|x| x.1.position().xy());
      }
      let ent = gen_bell(last_point, &ents, &mut bellc, &mut wst, &mut velc);
      self.last_created = Some(ent);
      ctl.last_round_gen.push(ent);
    }
  }
}

fn gen_bell<'a>(
  last_point: Option<Vec2>,
  entities: &Entities<'a>,
  bellc: &mut WriteStorage<'a, BellComponent>,
  wst: &mut WriteStorage<'a, WorldSpaceTransform>,
  velc: &mut WriteStorage<'a, Velocity>,
) -> Entity {
  let mut rng = rand::thread_rng();
  let new_y = last_point
    .map(|p| p.y + rng.gen_range(Y_VARIATION))
    .unwrap_or(LOWEST_Y);
  let dx = rng.sample::<f32, _>(StandardNormal) * X_STDDEV;
  let mut new_x = last_point.map(|v| v.x).unwrap_or(0f32) + dx;
  if new_x > STAGE_MAX_X - 1f32 {
    new_x = STAGE_MAX_X - 1f32;
  }
  if new_x < STAGE_MIN_X + 1f32 {
    new_x = STAGE_MIN_X + 1f32;
  }
  let point = Vec2::new(new_x, new_y);
  let ent = entities.create();
  bellc.insert(ent, BellComponent { size: BELL_SIZE });
  wst.insert(
    ent,
    WorldSpaceTransform::from_pos(point.extend(0f32))
      .add(Mat4::from_scale(Vec3::new(BELL_SIZE, BELL_SIZE, 1f32))),
  );
  velc.insert(ent, Velocity(Vec2::new(0f32, -BELL_Y_VEL)));
  ent
}

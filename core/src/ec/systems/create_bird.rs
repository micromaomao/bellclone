use std::borrow::Borrow;

use glam::Vec3;
use rand::{thread_rng, Rng};
use specs::{Entities, Read, System, WriteStorage};

use crate::{STAGE_MAX_X, STAGE_MIN_HEIGHT, STAGE_MIN_X, ec::components::{EntityId, bird::{Bird, Direction}, transform::WorldSpaceTransform}};

use super::max_player_y::MaxPlayerY;

use std::ops::Range;

pub const INIT_Y: f32 = 30f32;
pub const STEP_Y: Range<f32> = 25f32..35f32;

#[derive(Default)]
pub struct CreateBirdSystemController {
  pub enabled: bool
}

pub struct CreateBirdSystem {
  pub next_y: f32,
}

impl Default for CreateBirdSystem {
  fn default() -> Self {
    Self { next_y: INIT_Y }
  }
}

impl<'a> System<'a> for CreateBirdSystem {
  type SystemData = (
    Read<'a, CreateBirdSystemController>,
    Read<'a, MaxPlayerY>,
    WriteStorage<'a, Bird>,
    WriteStorage<'a, WorldSpaceTransform>,
    WriteStorage<'a, EntityId>,
    Entities<'a>,
  );
  fn run(&mut self, (ctl, max_player_y, mut birdc, mut wstc, mut entidc, ents): Self::SystemData) {
    if !ctl.enabled {
      return;
    }
    let max_player_y = max_player_y.borrow().0;
    if max_player_y.is_none() {
      return;
    }
    let max_player_y = max_player_y.unwrap();
    if max_player_y + STAGE_MIN_HEIGHT < self.next_y {
      return;
    }
    let mut rng = thread_rng();
    let x = rng.gen_range(STAGE_MIN_X + 1f32..STAGE_MAX_X - 1f32);
    let ent = ents.create();
    let dir = match rng.gen_bool(0.5f64) {
      false => Direction::LEFT,
      true => Direction::RIGHT,
    };
    birdc.insert(
      ent,
      Bird {
        direction: dir,
        turning: false,
      },
    );
    wstc.insert(ent, dir.into_wst_moving(Vec3::new(x, self.next_y, 0f32)));
    entidc.insert(ent, EntityId::new());
    self.next_y += rng.gen_range(STEP_Y);
  }
}

use std::ops::Range;

use glam::f32::*;
use rand::{Rng};
use rand_distr::StandardNormal;
use specs::{Builder, Entity, EntityBuilder, World};

use crate::{STAGE_MAX_X, STAGE_MIN_X, ec::components::bell::build_bell};

const LOWEST_Y: f32 = 2f32;
const INIT_X_VARIATION: f32 = 2f32;
const MAX_X_VARIATION: f32 = 5f32;
const Y_VARIATION: Range<f32> = 1.8f32..2.8f32;
const INIT_SIZE: f32 = 0.6f32;
const MIN_SIZE: f32 = 0.3f32;
const DIFFICULTY_RAISE_COUNTER: u32 = 50u32;

#[derive(Debug)]
pub struct BellGenContext {
  last_point: Vec2,
  x_variation_scaling: f32,
  bell_size: f32,
  next_difficulty_raise: u32,
}

impl BellGenContext {
  pub fn new() -> Self {
    BellGenContext {
      last_point: Vec2::ZERO,
      x_variation_scaling: 1f32,
      bell_size: INIT_SIZE,
      next_difficulty_raise: 0u32
    }
  }

  pub fn gen<F>(&mut self, world: &mut World, mut attach: F) -> Entity
  where
    F: FnMut(EntityBuilder) -> EntityBuilder,
  {
    let mut rng = rand::thread_rng();
    let new_y;
    if self.last_point.y < LOWEST_Y {
      new_y = LOWEST_Y;
    } else {
      new_y = self.last_point.y + rng.gen_range(Y_VARIATION);
    }
    let mut x_var = INIT_X_VARIATION * self.x_variation_scaling;
    if x_var > MAX_X_VARIATION {
      x_var = MAX_X_VARIATION;
    }
    let dx = rng.sample::<f32, _>(StandardNormal) * x_var;
    let mut new_x = self.last_point.x + dx;
    if new_x > STAGE_MAX_X - 1f32 {
      new_x = STAGE_MAX_X - 1f32;
    }
    if new_x < STAGE_MIN_X + 1f32 {
      new_x = STAGE_MIN_X + 1f32;
    }
    let point = Vec2::new(new_x, new_y);
    let ent = attach(build_bell(world, self.bell_size, point)).build();
    self.last_point = point;
    self.next_difficulty_raise += 1;
    if self.next_difficulty_raise >= DIFFICULTY_RAISE_COUNTER {
      self.next_difficulty_raise = 0u32;
      self.raise_difficulty();
    }
    ent
  }

  pub fn raise_difficulty(&mut self) {
    self.x_variation_scaling += 0.5f32;
    self.bell_size *= 0.9;
    if self.bell_size < MIN_SIZE {
      self.bell_size = MIN_SIZE;
    }
  }

  pub fn ensure<F>(&mut self, y: f32, world: &mut World, mut attach: F)
  where
    F: FnMut(EntityBuilder) -> EntityBuilder,
  {
    while self.last_point.y < y {
      self.gen(world, &mut attach);
    }
  }

  pub fn set_last_point(&mut self, point: Vec2) {
    self.last_point = point;
  }
}

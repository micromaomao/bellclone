use std::ops::Range;

use glam::f32::*;
use rand::{Rng};
use rand_distr::StandardNormal;
use specs::{Builder, Entity, EntityBuilder, World};

use crate::ec::components::bell::build_bell;

const LOWEST_Y: f32 = 2f32;
const X_VARIATION: f32 = 3f32;
const Y_VARIATION: Range<f32> = 1.8f32..2.8f32;

#[derive(Debug)]
pub struct BellGenContext {
  last_point: Vec2,
}

impl BellGenContext {
  pub fn new() -> Self {
    BellGenContext {
      last_point: Vec2::zero(),
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
    let dx = rng.sample::<f32, _>(StandardNormal) * X_VARIATION;
    let mut new_x = self.last_point.x + dx;
    if new_x > 7f32 {
      new_x = 7f32;
    }
    if new_x < -7f32 {
      new_x = -7f32;
    }
    let point = Vec2::new(new_x, new_y);
    let ent = attach(build_bell(world, 0.3f32, point)).build();
    self.last_point = point;
    ent
  }

  pub fn ensure<F>(&mut self, y: f32, world: &mut World, mut attach: F)
  where
    F: FnMut(EntityBuilder) -> EntityBuilder,
  {
    while self.last_point.y < y {
      self.gen(world, &mut attach);
    }
  }
}

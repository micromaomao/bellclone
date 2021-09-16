use glam::{Mat4, Vec3};
use specs::{Component, DenseVecStorage};

use crate::ec::DeltaTime;

use super::transform::WorldSpaceTransform;

use std::f32::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct Bird {
  pub direction: Direction,
  pub turning: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  LEFT,
  RIGHT,
}

impl Direction {
  pub fn reverse(self) -> Self {
    match self {
      Self::LEFT => Self::RIGHT,
      Self::RIGHT => Self::LEFT,
    }
  }

  pub fn into_wst_moving(self, pos: Vec3) -> WorldSpaceTransform {
    match self {
      Self::LEFT => WorldSpaceTransform::from_pos(pos).add(Mat4::from_rotation_y(PI)),
      Self::RIGHT => WorldSpaceTransform::from_pos(pos)
    }
  }

  pub fn turn_to_opposite_animation(self, dt: DeltaTime, tr: &mut WorldSpaceTransform) -> bool {
    const DURATION: f32 = 0.5f32;
    let step = PI * (dt.as_secs_f32() / DURATION);
    match self {
      Self::LEFT => {
        tr.add_to_self(Mat4::from_rotation_y(step));
        if tr.local_to_world(Vec3::X).z < 0f32 {
          *tr = Self::RIGHT.into_wst_moving(tr.position());
          return true;
        }
      },
      Self::RIGHT => {
        tr.add_to_self(Mat4::from_rotation_y(step));
        if tr.local_to_world(Vec3::X).z > 0f32 {
          *tr = Self::LEFT.into_wst_moving(tr.position());
          return true;
        }
      }
    }
    false
  }
}

impl Component for Bird {
  type Storage = DenseVecStorage<Bird>;
}

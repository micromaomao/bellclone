use game_core::{STAGE_MAX_X, STAGE_MIN_HEIGHT, STAGE_MIN_X, STAGE_WIDTH};
use glam::f32::*;

#[derive(Debug, Clone, Default)]
pub struct ViewportInfo {
  pub view_matrix: Mat4,
  pub tr: Vec2,
  pub bl: Vec2,
  pub width: u32,
  pub height: u32,
}

impl ViewportInfo {
  pub fn raycast(&self, pixel_x: u32, pixel_y: u32) -> Vec2 {
    Vec2::new(
      self.bl.x + pixel_x as f32 / self.width as f32 * (self.tr.x - self.bl.x),
      self.bl.y + pixel_y as f32 / self.height as f32 * (self.tr.y - self.bl.y),
    )
  }
}

pub fn solve_translation_scale(from1: Vec2, to1: Vec2, from2: Vec2, to2: Vec2) -> Mat3 {
  let xscale = (to2.x - to1.x) / (from2.x - from1.x);
  let yscale = (to2.y - to1.y) / (from2.y - from1.y);
  let xtrans = to1.x - from1.x * xscale;
  let ytrans = to1.y - from1.y * yscale;
  Mat3::from_scale_angle_translation(Vec2::new(xscale, yscale), 0f32, Vec2::new(xtrans, ytrans))
}

pub fn affine_2d_to_3d(transform: Mat3) -> Mat4 {
  let t = transform.to_cols_array_2d();
  let x = t[0];
  let y = t[1];
  let t = t[2];
  Mat4::from_cols_array_2d(&[
    [x[0], y[0], 0f32, t[0]],
    [x[1], y[1], 0f32, t[1]],
    [0f32, 0f32, 1f32, 0f32],
    [0f32, 0f32, 0f32, 1f32],
  ])
  .transpose()
}

pub fn view_matrix(width: u32, height: u32, camera_y: f32) -> ViewportInfo {
  let aspect_ratio = (width as f32) / (height as f32);
  let mut bl = Vec2::new(STAGE_MIN_X, camera_y);
  let mut tr = Vec2::new(STAGE_MAX_X, camera_y + STAGE_MIN_HEIGHT);
  const natural_aspect_ratio: f32 = STAGE_WIDTH / STAGE_MIN_HEIGHT;

  if aspect_ratio > natural_aspect_ratio {
    // space around
    let extra_width = (tr.y - bl.y) * aspect_ratio - (tr.x - bl.x);
    bl.x -= extra_width / 2f32;
    tr.x += extra_width / 2f32;
  } else if aspect_ratio < natural_aspect_ratio {
    // extend top
    let extra_height = (tr.x - bl.x) / aspect_ratio - (tr.y - bl.y);
    tr.y += extra_height;
  }

  let shift_and_scale =
    solve_translation_scale(bl, Vec2::new(-1f32, -1f32), tr, Vec2::new(1f32, 1f32));
  let aff4 = affine_2d_to_3d(shift_and_scale);
  let perspective_scale_factor = 0.2f32;
  let view_matrix = Mat4::from_cols_array_2d(&[
    [1f32, 0f32, 0f32, 0f32],
    [0f32, 1f32, 0f32, 0f32],
    [0f32, 0f32, 1f32, 0f32],
    [0f32, 0f32, perspective_scale_factor, 1f32],
  ])
  .transpose()
    * aff4;
  ViewportInfo {
    view_matrix,
    tr,
    bl,
    width,
    height,
  }
}

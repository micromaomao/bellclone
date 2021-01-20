use glam::f32::*;

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

pub fn view_matrix(aspect_ratio: f32) -> Mat4 {
  let bl = Vec2::new(-8f32, 0f32);
  let tr = Vec2::new(8f32, 9f32);

  // todo

  let shift_and_scale =
    solve_translation_scale(bl, Vec2::new(-1f32, -1f32), tr, Vec2::new(1f32, 1f32));
  let aff4 = affine_2d_to_3d(shift_and_scale);
  let perspective_scale_factor = 0.2f32;
  Mat4::from_cols_array_2d(&[
    [1f32, 0f32, 0f32, 0f32],
    [0f32, 1f32, 0f32, 0f32],
    [0f32, 0f32, 1f32, 0f32],
    [0f32, 0f32, perspective_scale_factor, 1f32],
  ])
  .transpose()
    * aff4
}

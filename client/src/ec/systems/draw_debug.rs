use game_core::ec::components::transform::WorldSpaceTransform;
use golem::{ElementBuffer, UniformValue, VertexBuffer};
use specs::{Join, Read, ReadStorage, System};

use crate::{ec::components::debug::DebugRect, log, render::DrawingCtx};

use glam::f32::*;

pub struct DrawDebug;
impl<'a> System<'a> for DrawDebug {
  type SystemData = (
    Read<'a, DrawingCtx>,
    ReadStorage<'a, DebugRect>,
    ReadStorage<'a, WorldSpaceTransform>,
  );

  fn run(&mut self, (dctx, debug_rects, transforms): Self::SystemData) {
    let mut shaders = dctx.shaders.borrow_mut();
    let prog = &mut shaders.debug_rect;
    prog.bind();
    let mut buf = VertexBuffer::new(dctx.glctx).unwrap();
    buf.set_data(&[
      -1f32, -1f32, // bl
      1f32, -1f32, // br
      1f32, 1f32, // tr
      -1f32, 1f32, // tl
    ]);
    let mut ele = ElementBuffer::new(dctx.glctx).unwrap();
    ele.set_data(&[
      0, 1, 1, 2, 2, 3, 3, 0, // box
      3, 1, // \
      0, 2, // /
    ]);
    prog.set_uniform(
      "uViewMat",
      UniformValue::Matrix4(dctx.view_mat.to_cols_array()),
    );
    prog.prepare_draw(&buf, &ele).unwrap();
    for (rect, tr) in (&debug_rects, &transforms).join() {
      prog.set_uniform(
        "uObjectTransform",
        UniformValue::Matrix4(tr.0.to_cols_array()),
      );
      prog.set_uniform("uSize", UniformValue::Vector2([rect.width, rect.height]));
      unsafe { prog.draw_prepared(0..12, golem::GeometryMode::Lines) };
    }
  }
}

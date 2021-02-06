use game_core::ec::components::transform::WorldSpaceTransform;
use golem::{Context, ElementBuffer, UniformValue, VertexBuffer};
use specs::{Join, Read, ReadStorage, System};

use crate::{ec::components::debug::DebugRect, render::DrawingCtx};

pub struct DrawDebug {
  buf: VertexBuffer,
  ele: ElementBuffer,
}

impl DrawDebug {
  pub fn new(glctx: &Context) -> Result<Self, golem::GolemError> {
    let mut buf = VertexBuffer::new(glctx).unwrap();
    buf.set_data(&[
      -1f32, -1f32, // bl
      1f32, -1f32, // br
      1f32, 1f32, // tr
      -1f32, 1f32, // tl
    ]);
    let mut ele = ElementBuffer::new(glctx).unwrap();
    ele.set_data(&[
      0, 1, 1, 2, 2, 3, 3, 0, // box
      3, 1, // \
      0, 2, // /
    ]);
    Ok(Self {
      buf, ele
    })
  }
}

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
    prog.prepare_draw(&self.buf, &self.ele).unwrap();
    for (rect, tr) in (&debug_rects, &transforms).join() {
      prog
        .set_uniform(
          "uTransform",
          UniformValue::Matrix4((dctx.viewport.view_matrix * tr.0).to_cols_array()),
        )
        .unwrap();
      prog
        .set_uniform("uSize", UniformValue::Vector2([rect.width, rect.height]))
        .unwrap();
      unsafe { prog.draw_prepared(0..12, golem::GeometryMode::Lines) };
    }
  }
}

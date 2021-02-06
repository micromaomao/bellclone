use std::num::NonZeroU32;

use game_core::ec::components::transform::WorldSpaceTransform;
use golem::{Context, ElementBuffer, UniformValue, VertexBuffer};
use specs::{Join, Read, ReadStorage, System};

use crate::{ec::components::{DrawImage, player::OurPlayer}, render::DrawingCtx};

pub struct DrawImageSystem {
  buf: VertexBuffer,
  ele: ElementBuffer,
}

impl DrawImageSystem {
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
      1, 2, 0, // /|
      3, // |/
    ]);
    Ok(Self {
      buf, ele
    })
  }
}

impl<'a> System<'a> for DrawImageSystem {
  type SystemData = (
    Read<'a, DrawingCtx>,
    ReadStorage<'a, WorldSpaceTransform>,
    ReadStorage<'a, DrawImage>,
    ReadStorage<'a, OurPlayer>,
  );

  fn run(&mut self, (dctx, trs, imgs, ops): Self::SystemData) {
    let mut shaders = dctx.shaders.borrow_mut();
    let prog = &mut shaders.image;
    prog.bind();
    prog.set_uniform("tex", UniformValue::Int(1)).unwrap();
    prog.prepare_draw(&self.buf, &self.ele).unwrap();
    macro_rules! d {
      ($tr:ident, $img:ident) => {
        prog
          .set_uniform(
            "uTransform",
            UniformValue::Matrix4((dctx.viewport.view_matrix * $tr.0).to_cols_array()),
          )
          .unwrap();
        prog
          .set_uniform("alpha", UniformValue::Float($img.alpha))
          .unwrap();
        $img
          .texture
          .gl_texture
          .set_active(NonZeroU32::new(1).unwrap());
        prog
          .set_uniform("uSize", UniformValue::Vector2([$img.size.x, $img.size.y]))
          .unwrap();
        unsafe { prog.draw_prepared(0..4, golem::GeometryMode::TriangleStrip) };
      };
    }
    // so that we draw our player above anything else.
    for (tr, img, _) in (&trs, &imgs, !&ops).join() {
      d!(tr, img);
    }
    for (tr, img, _) in (&trs, &imgs, &ops).join() {
      d!(tr, img);
    }
  }
}

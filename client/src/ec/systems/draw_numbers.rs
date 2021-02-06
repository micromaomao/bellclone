use std::num::NonZeroU32;

use game_core::ec::components::transform::WorldSpaceTransform;
use glam::f32::*;
use golem::{Context, ElementBuffer, UniformValue};
use specs::{Join, Read, ReadStorage, System};

use crate::{
  ec::components::draw_numbers::{construct_elem_buf, Align, DrawNumbersComponent},
  render::DrawingCtx,
};

pub struct DrawNumbersSystem {
  cached_element_buf: ElementBuffer,
}

impl DrawNumbersSystem {
  pub fn new(glctx: &Context) -> Result<Self, golem::GolemError> {
    Ok(Self {
      cached_element_buf: construct_elem_buf(glctx)?,
    })
  }
}

impl<'a> System<'a> for DrawNumbersSystem {
  type SystemData = (
    Read<'a, DrawingCtx>,
    ReadStorage<'a, DrawNumbersComponent>,
    ReadStorage<'a, WorldSpaceTransform>,
  );

  fn run(&mut self, (dctx, dncs, trs): Self::SystemData) {
    let mut shaders = dctx.shaders.borrow_mut();
    let tex = &dctx.images.numbers.gl_texture;
    tex.set_active(NonZeroU32::new(1).unwrap());
    let prog = &mut shaders.simple2d;
    prog.bind();
    prog.set_uniform("tex", UniformValue::Int(1)).unwrap();
    // so that we draw our player above anything else.
    for (dn, tr) in (&dncs, &trs).join() {
      let xshift = match dn.align {
        Align::Left => 0f32,
        Align::Center => dn.nb_digits() as f32 * -0.5f32,
      };
      prog
        .set_uniform(
          "uTransform",
          UniformValue::Matrix4(
            (dctx.viewport.view_matrix * tr.0 * Mat4::from_translation(Vec3::unit_x() * xshift))
              .to_cols_array(),
          ),
        )
        .unwrap();
      prog
        .set_uniform("alpha", UniformValue::Float(dn.alpha()))
        .unwrap();
      unsafe {
        prog
          .draw(
            dn.buf(),
            &self.cached_element_buf,
            0..dn.draw_size(),
            golem::GeometryMode::Triangles,
          )
          .unwrap();
      }
    }
  }
}

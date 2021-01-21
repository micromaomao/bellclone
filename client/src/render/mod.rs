use core::panic;
use std::cell::RefCell;

use glam::f32::*;
use golem::{blend::BlendMode, glow};
use shader_program::Shaders;
use view::affine_2d_to_3d;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub mod image_texture;
pub mod shader_program;
pub mod view;

pub struct GraphicsCtx {
  pub glctx: golem::Context,
  pub canvas: HtmlCanvasElement,
  pub aspect_ratio: RefCell<f32>,
  pub viewport_size: RefCell<(u32, u32)>,
  pub shaders: RefCell<Shaders>,
}

#[derive(Clone)]
pub struct DrawingCtx {
  pub glctx: &'static golem::Context,
  pub view_mat: Mat4,
  pub shaders: &'static RefCell<Shaders>,
}

impl Default for DrawingCtx {
  fn default() -> Self {
    panic!("Not possible.");
  }
}

impl DrawingCtx {
  pub fn zoom_in(self, bottom_left: Vec2, top_right: Vec2, width: f32, height: f32) -> Self {
    self.prepend_mat(affine_2d_to_3d(view::solve_translation_scale(
      Vec2::new(0f32, 0f32),
      bottom_left,
      Vec2::new(width, height),
      top_right,
    )))
  }

  pub fn prepend_mat(self, transform: Mat4) -> Self {
    Self {
      glctx: self.glctx,
      view_mat: self.view_mat * transform,
      shaders: self.shaders,
    }
  }
}

impl GraphicsCtx {
  pub fn init() -> Result<Self, golem::GolemError> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let ele: HtmlCanvasElement = document.create_element("canvas").unwrap().unchecked_into();
    web_sys::Node::from(document.body().unwrap())
      .append_child(&ele)
      .unwrap();
    let glctx = golem::Context::from_glow(glow::Context::from_webgl1_context(
      ele
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap(),
    ))?;

    let shaders = Shaders::load(&glctx)?;

    Ok(GraphicsCtx {
      glctx,
      canvas: ele,
      aspect_ratio: RefCell::new(0f32),
      viewport_size: RefCell::new((0u32, 0u32)),
      shaders: RefCell::new(shaders),
    })
  }

  pub fn resize(&self, width: u32, height: u32) {
    let window = web_sys::window().unwrap();
    self.canvas.set_width(width);
    self.canvas.set_height(height);
    self.aspect_ratio.replace(width as f32 / height as f32);
    self.viewport_size.replace((width, height));
    self.glctx.set_viewport(0, 0, width, height);
  }

  pub fn prepare_render(&'static self, view_matrix: Mat4) -> DrawingCtx {
    let aspect_ratio = *self.aspect_ratio.borrow();
    if aspect_ratio == 0f32 {
      panic!("Size not initalized yet.");
    }

    let glctx = &self.glctx;

    glctx.set_blend_mode(Some(BlendMode::default()));
    glctx.set_depth_test_mode(None);
    glctx.set_clear_color(0f32, 0f32, 0f32, 1f32);
    glctx.clear();

    DrawingCtx {
      glctx,
      view_mat: view_matrix,
      shaders: &self.shaders,
    }
  }
}

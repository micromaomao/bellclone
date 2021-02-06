use core::panic;
use std::{cell::RefCell, error::Error};

use glam::f32::*;
use golem::{blend::BlendMode, glow};
use js_sys::Object;
use js_sys::Reflect;
use shader_program::Shaders;
use view::{affine_2d_to_3d, ViewportInfo};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

pub mod image_texture;
pub mod shader_program;
pub mod view;

pub struct GraphicsCtx {
  pub glctx: golem::Context,
  pub canvas: HtmlCanvasElement,
  pub viewport_size: RefCell<(u32, u32)>,
  pub shaders: RefCell<Shaders>,
  pub images: image_texture::Images,
}

#[derive(Clone)]
pub struct DrawingCtx {
  pub glctx: &'static golem::Context,
  pub viewport: ViewportInfo,
  pub shaders: &'static RefCell<Shaders>,
  pub images: &'static image_texture::Images,
}

impl Default for DrawingCtx {
  fn default() -> Self {
    panic!("Not possible.");
  }
}

impl GraphicsCtx {
  pub fn init() -> Result<Self, Box<dyn Error>> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let ele: HtmlCanvasElement = document.create_element("canvas").unwrap().unchecked_into();
    web_sys::Node::from(document.body().unwrap())
      .append_child(&ele)
      .unwrap();
    let glopt = Object::new();
    Reflect::set(&glopt, &JsValue::from_str("alpha"), &JsValue::from_bool(false)).unwrap();
    let glctx = golem::Context::from_glow(glow::Context::from_webgl1_context(
      ele
        .get_context_with_context_options("webgl", &glopt)
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap(),
    ))
    .map_err(|e| e.to_string())?;

    let shaders = Shaders::load(&glctx).map_err(|e| e.to_string())?;
    let images = image_texture::Images::load(&glctx)?;

    Ok(GraphicsCtx {
      glctx,
      canvas: ele,
      viewport_size: RefCell::new((0u32, 0u32)),
      shaders: RefCell::new(shaders),
      images,
    })
  }

  pub fn resize(&self, width: u32, height: u32) {
    let window = web_sys::window().unwrap();
    self.canvas.set_width(width);
    self.canvas.set_height(height);
    self.viewport_size.replace((width, height));
    self.glctx.set_viewport(0, 0, width, height);
  }

  pub fn prepare_render(&'static self, viewport: ViewportInfo) -> DrawingCtx {
    let glctx = &self.glctx;

    glctx.set_blend_mode(Some(BlendMode::default()));
    glctx.set_depth_test_mode(None);
    // glctx.set_clear_color(0.9765f32, 0.9686f32, 0.9255f32, 1f32);
    glctx.set_clear_color(51f32/255f32, 0f32, 102f32/255f32, 1f32);
    glctx.clear();

    DrawingCtx {
      glctx,
      viewport,
      shaders: &self.shaders,
      images: &self.images,
    }
  }
}

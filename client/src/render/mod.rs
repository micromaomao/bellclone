use std::mem::MaybeUninit;

use golem::glow;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, prelude::*};
use web_sys::HtmlCanvasElement;

pub struct GraphicsCtx {
  glctx: golem::Context,
  canvas: HtmlCanvasElement,
}

impl GraphicsCtx {
  pub fn init() -> Self {
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
    ))
    .unwrap();

    GraphicsCtx { glctx, canvas: ele }
  }

  pub fn resize(&self, width: u32, height: u32) {
    let window = web_sys::window().unwrap();
    self.canvas.set_width(width);
    self.canvas.set_height(height);
  }

  pub fn render(&self) {
    let glctx = &self.glctx;

    glctx.set_clear_color(0f32, 0f32, 0f32, 1f32);
    glctx.clear();
  }
}

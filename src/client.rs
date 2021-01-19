use std::mem::MaybeUninit;

use golem::glow;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, prelude::*};
use web_sys::HtmlCanvasElement;

struct Context {
  graphics: GraphicsCtx,
}
static mut game_ctx: MaybeUninit<Context> = MaybeUninit::uninit();

#[wasm_bindgen(start)]
pub fn client_init() {
  unsafe {
    let graphics = init_graphics();
    game_ctx = MaybeUninit::new(Context { graphics });

    let window = web_sys::window().unwrap();
    window.add_event_listener_with_callback(
      "resize",
      Closure::wrap(Box::new(handle_resize) as Box<dyn Fn()>).into_js_value().dyn_ref().unwrap(),
    );

    handle_resize();
  }
}

struct GraphicsCtx {
  glctx: golem::Context,
  canvas: HtmlCanvasElement,
}

fn init_graphics() -> GraphicsCtx {
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

fn handle_resize() {
  let gr = unsafe { &(&*game_ctx.as_ptr()).graphics };
  let window = web_sys::window().unwrap();
  gr.canvas
    .set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
  gr.canvas
    .set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

  render();
}

fn render() {
  let gr = unsafe { &(&*game_ctx.as_ptr()).graphics };
  let glctx = &gr.glctx;

  glctx.set_clear_color(0f32, 0f32, 0f32, 1f32);
  glctx.clear();
}

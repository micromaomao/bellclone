use global::Context;
use render::GraphicsCtx;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, prelude::*};

mod global;
mod render;

#[wasm_bindgen(start)]
pub fn client_init() {
  unsafe {
    let graphics = GraphicsCtx::init();
    global::init_ctx(Context { graphics });

    let window = web_sys::window().unwrap();
    window.add_event_listener_with_callback(
      "resize",
      Closure::wrap(Box::new(handle_resize) as Box<dyn Fn()>)
        .into_js_value()
        .dyn_ref()
        .unwrap(),
    );

    handle_resize();
  }
}

fn handle_resize() {
  let gr = &global::get_ref().graphics;
  let window = web_sys::window().unwrap();
  let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
  let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
  gr.resize(width, height);
}

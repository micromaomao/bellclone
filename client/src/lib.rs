use core::panic;
use std::cell::RefCell;

use ec::EcCtx;
use global::Context;
use render::GraphicsCtx;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, prelude::*};
use world_manager::WorldManager;

mod ec;
mod global;
mod render;
mod webapi_utils;
mod world_manager;

#[macro_export]
macro_rules! log {
  ($($format_args:expr),+) => {
    ::web_sys::console::log_1(&::wasm_bindgen::JsValue::from_str(&format!($($format_args),+)));
  };
}

#[wasm_bindgen(start)]
pub fn client_init() {
  std::panic::set_hook(Box::new(console_error_panic_hook::hook));
  unsafe {
    let graphics = match GraphicsCtx::init() {
      Ok(k) => k,
      Err(e) => {
        panic!("Error initalizing graphics: {}", e)
      }
    };
    let ec = RefCell::new(EcCtx::new());
    let we = RefCell::new(WorldManager::new(&mut *ec.borrow_mut()));
    global::init_ctx(Context {
      graphics,
      ec,
      world_manager: we,
    });

    let window = web_sys::window().unwrap();
    window
      .add_event_listener_with_callback(
        "resize",
        Closure::wrap(Box::new(handle_resize) as Box<dyn Fn()>)
          .into_js_value()
          .dyn_ref()
          .unwrap(),
      )
      .unwrap();
    window
      .add_event_listener_with_callback(
        "mousemove",
        Closure::wrap(Box::new(handle_mousedown) as Box<dyn Fn(JsValue)>)
          .into_js_value()
          .dyn_ref()
          .unwrap(),
      )
      .unwrap();

    handle_resize();
    handle_redraw();
  }
}

fn handle_resize() {
  let mut gr = &global::get_ref().graphics;
  let window = web_sys::window().unwrap();
  let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
  let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
  gr.resize(width, height);
}

fn handle_mousedown(evt: JsValue) {
  let evt = evt.dyn_into::<web_sys::MouseEvent>().unwrap();
  let point: (i32, i32) = (evt.client_x(), evt.client_y());
  if point.0 < 0 || point.1 < 0 {
    unreachable!();
  }
  let point: (u32, u32) = (point.0 as u32, point.1 as u32);
  let global = global::get_ref();
  let mut ec = global.ec.borrow_mut();
  ec.pointer_state_mut().update_pos(point);
}

fn handle_redraw() {
  let global = global::get_ref();
  let ec = &global.ec;
  let mut ec = ec.borrow_mut();
  ec.update();
  let viewport;
  let gr = &global.graphics;
  {
    let mut wm = global.world_manager.borrow_mut();
    wm.update(&mut ec);
    let size = *gr.viewport_size.borrow();
    viewport = wm.view_matrix(&ec, size.0, size.1);
  }
  ec.pointer_state_mut().recalculate_raycast(&viewport);
  let dctx = gr.prepare_render(viewport);
  ec.render(dctx);

  let window = web_sys::window().unwrap();
  window
    .request_animation_frame(
      Closure::wrap(Box::new(handle_redraw) as Box<dyn Fn()>)
        .into_js_value()
        .dyn_ref()
        .unwrap(),
    )
    .unwrap();
}

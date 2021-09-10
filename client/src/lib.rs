use core::panic;
use std::{cell::RefCell, unreachable};

use ec::EcCtx;
use global::Context;
use render::{GraphicsCtx, ViewportSize};
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, prelude::*};
use web_sys::{AddEventListenerOptions, MouseEvent, TouchEvent};
use websocket::SocketContext;
use world_manager::WorldManager;

mod ec;
mod enc;
mod global;
mod render;
mod webapi_utils;
mod websocket;
mod world_manager;

pub const DEFAULT_GAME_SERVER: &str = "ws://127.0.0.1:5000";

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
    let ec = RefCell::new(EcCtx::new(&graphics));
    let wm = RefCell::new(WorldManager::new(&mut *ec.borrow_mut()));
    let socket_context = SocketContext::new();
    global::init_ctx(Context {
      graphics,
      ec,
      world_manager: wm,
      socket_context,
    });
    let global = global::get_ref();
    global.socket_context.connect(DEFAULT_GAME_SERVER);

    let window = web_sys::window().unwrap();
    let mut nopassive_opt = AddEventListenerOptions::new();
    nopassive_opt.passive(false);
    let events: &[(&[&str], fn(JsValue))] = &[
      (&["resize"], handle_resize_evt),
      (&["mousemove", "touchmove"], handle_pointer_move),
      (&["mousedown", "touchstart"], handle_pointer_down),
      (&["mouseup", "touchend", "touchcancel"], handle_pointer_up),
    ];
    for (evs, handler) in events.iter() {
      for &ev in evs.iter() {
        window
          .add_event_listener_with_callback_and_add_event_listener_options(
            ev,
            Closure::wrap(Box::new(handler) as Box<dyn Fn(JsValue)>)
              .into_js_value()
              .dyn_ref()
              .unwrap(),
            &nopassive_opt,
          )
          .unwrap();
      }
    }

    handle_resize();
    handle_redraw();
  }
}

fn handle_resize() {
  let gr = &global::get_ref().graphics;
  let window = web_sys::window().unwrap();
  let width = window.inner_width().unwrap().as_f64().unwrap();
  let height = window.inner_height().unwrap().as_f64().unwrap();
  let real_width = width * window.device_pixel_ratio();
  let real_height = height * window.device_pixel_ratio();
  gr.resize(ViewportSize {
    width: width as u32,
    height: height as u32,
    real_width: real_width as u32,
    real_height: real_height as u32,
  });
  let mut ec = global::get_ref().ec.borrow_mut();
  ec.resize(gr);
}

fn handle_resize_evt(_evt: JsValue) {
  handle_resize();
}

fn get_point_from_pointer_evt(evt: JsValue) -> Option<(u32, u32)> {
  let mut point: (i32, i32);
  match evt.dyn_into::<MouseEvent>() {
    Ok(evt) => {
      evt.prevent_default();
      point = (evt.client_x(), evt.client_y());
      if point.0 < 0 {
        point.0 = 0;
      }
      if point.1 < 0 {
        point.1 = 0;
      }
    }
    Err(evt) => match evt.dyn_into::<TouchEvent>() {
      Ok(evt) => {
        evt.prevent_default();
        let touches = evt.touches();
        if touches.length() != 1 {
          return None;
        }
        let t = touches.item(0).unwrap();
        point = (t.client_x(), t.client_y());
      }
      Err(_) => unreachable!(),
    },
  }
  Some((point.0 as u32, point.1 as u32))
}

fn handle_pointer_move(evt: JsValue) {
  let global = global::get_ref();
  let mut ec = global.ec.borrow_mut();
  match get_point_from_pointer_evt(evt) {
    Some(point) => {
      ec.pointer_state_mut().pointer_move(point);
    }
    None => {
      ec.pointer_state_mut().up();
    }
  }
}

fn handle_pointer_down(evt: JsValue) {
  let global = global::get_ref();
  let mut ec = global.ec.borrow_mut();
  let mut ps = ec.pointer_state_mut();
  match get_point_from_pointer_evt(evt) {
    Some(point) => {
      ps.pointer_move(point);
      ps.down();
    }
    None => {
      ps.up();
    }
  }
}

fn handle_pointer_up(_evt: JsValue) {
  let global = global::get_ref();
  let mut ec = global.ec.borrow_mut();
  ec.pointer_state_mut().up();
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
    viewport = wm.calculate_camera(&ec, size);
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

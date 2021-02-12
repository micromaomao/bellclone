use std::cell::RefCell;

use js_sys::{Uint8Array};
use protocol::servermsg_generated;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{BinaryType, MessageEvent, WebSocket};

use crate::global;


pub struct SocketContext {
  connection_id: RefCell<u32>,
  ws_obj: RefCell<Option<WebSocket>>,
  msgbuf: RefCell<Vec<u8>>,
}

macro_rules! wm_and_ec {
  ($wm:ident,$ec:ident) => {
    let mut wm = global::get_ref().world_manager.borrow_mut();
    let mut ec = global::get_ref().ec.borrow_mut();
    let $wm = &mut *wm;
    let $ec = &mut *ec;
  };
}

impl SocketContext {
  pub fn new() -> Self {
    SocketContext {
      connection_id: RefCell::new(0),
      ws_obj: RefCell::new(None),
      msgbuf: RefCell::new(Vec::new()),
    }
  }

  pub fn connect(&'static self, server: &str) {
    let mut connection_id = self.connection_id.borrow_mut();
    let mut ws_obj = self.ws_obj.borrow_mut();
    *connection_id += 1;
    *ws_obj = Some(WebSocket::new(server).unwrap());
    let ws_obj = ws_obj.as_ref().unwrap();
    let connection_id = *connection_id;
    ws_obj.set_binary_type(BinaryType::Arraybuffer);
    ws_obj.set_onopen(
      Closure::wrap(Box::new(move || {
        if self.check_id(connection_id) {
          self.onopen();
        }
      }) as Box<dyn Fn()>)
      .into_js_value()
      .dyn_ref(),
    );
    ws_obj.set_onclose(
      Closure::wrap(Box::new(move || {
        if self.check_id(connection_id) {
          self.onclose();
        }
      }) as Box<dyn Fn()>)
      .into_js_value()
      .dyn_ref(),
    );
    ws_obj.set_onmessage(
      Closure::wrap(Box::new(move |evt: JsValue| {
        if self.check_id(connection_id) {
          self.onmessage(evt.dyn_into().unwrap());
        }
      }) as Box<dyn Fn(JsValue)>)
      .into_js_value()
      .dyn_ref(),
    );
    ws_obj.set_onerror(
      Closure::wrap(Box::new(move || {
        if self.check_id(connection_id) {
          self.onerror();
        }
      }) as Box<dyn Fn()>)
      .into_js_value()
      .dyn_ref(),
    );
  }

  fn check_id(&self, expected_id: u32) -> bool {
    *self.connection_id.borrow() == expected_id
  }

  fn onopen(&'static self) {
    wm_and_ec!(wm, ec);
    wm.init_online(ec);
  }
  fn onclose(&'static self) {
    wm_and_ec!(wm, ec);
    wm.show_connection_error(ec);
    wm.init_offline(ec);
  }
  fn onmessage(&'static self, evt: MessageEvent) {
    wm_and_ec!(wm, ec);
    let data = Uint8Array::new(&evt.data());
    let mut msgbuf = self.msgbuf.borrow_mut();
    msgbuf.resize(data.byte_length() as usize, 0u8);
    data.copy_to(&mut msgbuf[..]);
    let msg = servermsg_generated::get_root_as_server_message(&msgbuf[..]);
    wm.process_msg(ec, msg);
  }
  fn onerror(&'static self) {
    wm_and_ec!(wm, ec);
    wm.show_connection_error(ec);
    wm.init_offline(ec);
  }
}

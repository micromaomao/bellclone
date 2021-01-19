use wasm_bindgen::prelude::*;

use std::convert::Into;

#[wasm_bindgen(start)]
pub fn client_init() {
  web_sys::console::log_1(&"Hello world!".into());
}

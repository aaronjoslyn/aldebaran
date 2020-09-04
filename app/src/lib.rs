#[macro_use]
mod util;
mod websocket;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    websocket::connect_websocket()
}

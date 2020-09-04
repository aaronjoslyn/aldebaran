use wasm_bindgen::prelude::*;

macro_rules! console_log {
    ($($t:tt)*) => (crate::util::log(&format_args!($($t)*).to_string()))
}

macro_rules! console_error {
    ($($t:tt)*) => (crate::util::error(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn error(s: &str);
}

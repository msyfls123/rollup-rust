use wasm_bindgen::prelude::*;
use web_sys::{console, window};
use js_sys::{Reflect};
use serde_json::{Value};
use serde::{Deserialize, Serialize};

mod clock;
mod file;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(start)]
pub fn main_js() {
    console_error_panic_hook::set_once();
}

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&JsValue::from(&format!("Hello, {}!", name)));
}

#[wasm_bindgen]
pub fn keys(data: &JsValue) {
    let keys = Reflect::own_keys(data).unwrap_throw();
    for key in keys.iter() {
        console::log_1(&key);
    }
}

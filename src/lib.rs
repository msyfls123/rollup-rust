use wasm_bindgen::prelude::*;
use web_sys::{console, window};

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
    console::log_1(&JsValue::from("Hello world!!!"));
    let window = window().unwrap();
    window.alert_with_message(&format!("Hello, {}!", name));
}

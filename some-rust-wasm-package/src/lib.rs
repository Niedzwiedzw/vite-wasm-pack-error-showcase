mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (unsafe {log(&format_args!($($t)*).to_string())})
}

#[wasm_bindgen]
pub fn greet(i32_parameter: i32, string_parameter: String, js_value_parameter: JsValue) {
    utils::set_panic_hook();
    console_log!("i32_parameter: {i32_parameter:?}, string_parameter: {string_parameter:?}, js_value_parameter: {js_value_parameter:#?}")
}

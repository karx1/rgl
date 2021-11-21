use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setInterval)]
    pub fn set_interval(closure: &Closure<dyn Fn()>, time: u32) -> i32;
    #[wasm_bindgen(js_name = clearInterval)]
    pub fn clear_interval(id: i32);
}
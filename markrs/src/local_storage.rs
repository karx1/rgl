use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localStorage, js_name = setItem)]
    pub fn set_item(key: &str, value: &str);
    #[wasm_bindgen(js_namespace = localStorage, js_name = getItem)]
    pub fn get_item(key: &str) -> Option<String>;
}

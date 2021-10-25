use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = listLocalStorageKeys)]
    pub fn list_local_storage_keys() -> js_sys::Array;

    #[wasm_bindgen(js_namespace = localStorage, js_name = setItem)]
    pub fn set_item(key: &str, value: &str);

    #[wasm_bindgen(js_namespace = localStorage, js_name = getItem)]
    pub fn get_item(key: &str) -> String;

    #[wasm_bindgen(js_namespace = localStorage, js_name = removeItem)]
    pub fn remove_item(key: &str);
}

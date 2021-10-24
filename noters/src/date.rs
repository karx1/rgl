use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = getCurrentTimeMillis)]
    pub fn get_current_time_millis() -> u64;

    #[wasm_bindgen(js_name = timeHR)]
    pub fn time_hr(millis: u64) -> String;
}

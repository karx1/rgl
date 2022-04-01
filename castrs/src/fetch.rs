use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub async fn make_request<S: AsRef<str>>(endpoint: S, method: S) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(method.as_ref());
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(endpoint.as_ref(), &opts)?;

    // request
    //    .headers()
    //

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

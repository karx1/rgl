use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde_json::Value;
use crate::types::*;

pub async fn make_request<S: AsRef<str>>(endpoint: S, method: S) -> Result<Vec<Returned>, JsValue> {
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

    let v = json.into_serde::<Value>().unwrap()["results"].as_array().to_owned();

    let mut new: Vec<Returned> = Vec::with_capacity(v.len());

    for item in v {
        let e = serde_json::from_value::<Episode>(item.clone());
        let p = serde_json::from_value::<Podcast>(item.clone());
        let c = serde_json::from_value::<Curated>(item.clone());

        if let Ok(e) = e {
            new.push(Returned::Episode(e));
        } else if let Ok(p) = p { 
            new.push(Returned::Podcast(p));
        } else if let Ok(c) = c {
            new.push(c);
        }
    }

    Ok(new)
}

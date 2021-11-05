use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;

fn main() {
    let state = Signal::new(String::new());
    spawn_local(cloned!((state) => async move {
        let resp = surf::get(env!("FIREBASE_URL")).recv_string().await.unwrap();
        let parsed = json::parse(&resp).unwrap();

        state.set(parsed["votes"].to_string());
    }));
    sycamore::render(|| {
        template! {
            div(class="wrapper") {
                p { (state.get()) }
            }
        }
    });
}
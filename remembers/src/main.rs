use sycamore::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Event, Element};

macro_rules! wasm_import {
    ($($tt:tt)*) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $($tt)*;
        }
    };
}

macro_rules! wasm_import_with_ns {
    ($ns: ident, $($tt:tt)*) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = $ns)]
            pub fn $($tt)*;
        }
    };
}

wasm_import!(toggle_cards());
wasm_import_with_ns!(console, log(s: &str));

fn main() {
    sycamore::render(|ctx| {
        console_error_panic_hook::set_once();

        let on_click = |event: Event| {
            let elem = event.target().unwrap().dyn_ref::<Element>().unwrap().clone();
            elem.class_list().toggle("flip").unwrap();
        };

        view! {ctx, 
        div(class="wrapper") {
            h1(class="text-align-center") { "RemembeRS" }
            div(id="game") {
                div(class="card", on:click=on_click) {
                    "Hello, world!"
                }
            }
        }
    }});
}

use sycamore::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, Event};

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

        let v: &Signal<Vec<u8>> = ctx.create_signal((0..12u8).collect());

        let on_click = |event: Event| {
            let elem = event
                .current_target()
                .unwrap()
                .dyn_ref::<Element>()
                .unwrap()
                .clone();
            elem.class_list().toggle("flip").unwrap();
        };

        view! {ctx,
            div(class="wrapper") {
                h1(class="text-align-center") { "RemembeRS" }
                section(id="game") {
                    Keyed {
                        iterable: v,
                        view: move |ctx, i| view! {ctx,
                            div(class="card", on:click=on_click) {
                                h2(class="back-face") {
                                    (i)
                                }
                                div(class="front-face")
                            }
                        },
                        key: |x| *x
                    }
                }
            }
        }
    });
}

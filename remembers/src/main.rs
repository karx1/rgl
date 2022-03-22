use rand::seq::SliceRandom;
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

wasm_import!(setTimeout(closure: &Closure<dyn Fn()>, time: u32));

fn main() {
    sycamore::render(|ctx| {
        console_error_panic_hook::set_once();

        let mut cards = ["burger", "fries", "hotdog", "soda", "nachos", "tacos"]
            .into_iter()
            .cycle()
            .take(12)
            // .map(|s| s.to_string())
            .collect::<Vec<_>>();

        cards.shuffle(&mut rand::rngs::OsRng);

        let v = ctx.create_signal(cards);

        let first: &Signal<Option<Element>> = ctx.create_signal(None);

        let on_click = |event: Event| {
            let elem = event
                .current_target()
                .unwrap()
                .dyn_ref::<Element>()
                .unwrap()
                .clone();

            if elem.class_list().contains("disabled") {
                return;
            }

            elem.class_list().toggle("flip").unwrap();

            if let Some(ref felem) = *first.get() {
                let attr1 = felem.get_attribute("data_value");
                let attr2 = elem.get_attribute("data_value");

                if attr1 == attr2 {
                    // these are all ok to unwrap, because it should never ever fail
                    felem.class_list().toggle("disabled").unwrap();
                    elem.class_list().toggle("disabled").unwrap();
                } else {
                    let felem = felem.clone();
                    let cb = Closure::wrap(Box::new(move || {
                        felem.class_list().toggle("flip").unwrap();
                        elem.class_list().toggle("flip").unwrap();
                    }) as Box<dyn Fn()>);

                    setTimeout(&cb, 750);

                    cb.forget();
                }

                first.set(None);
            } else {
                first.set(Some(elem));
            }
        };

        view! {ctx,
            div(class="wrapper") {
            h1(class="text-align-center") { "RemembeRS" }
            section(id="game") {
                Keyed {
                    iterable: v,
                    view: move |ctx, i| view! {ctx,
                        div(class="card", on:click=on_click, data_value=i.clone()) {
                            h2(class="back-face") {
                                (i)
                            }
                            div(class="front-face")
                        }
                    },
                    key: |x| x.clone()
                }
            }
            }
        }
    });
}
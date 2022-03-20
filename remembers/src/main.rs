use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

macro_rules! wasm_import {
    ($($tt:tt)*) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $($tt)*;
        }
    };
    ($ns: ident, $($tt:tt)*) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = $ns)]
            pub fn $($tt)*;
        }
    };
    ($name: ident, $($tt:tt)*) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_name = $name)]
            pub fn $($tt)*;
        }
    };
    ($name:ident, $ns: ident, $($tt:tt)*) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_name = $name, js_namespace = $ns)]
            pub fn $($tt)*;
        }
    };
}

wasm_import!(toggle_cards());

fn main() {
    sycamore::render(|ctx| {
        console_error_panic_hook::set_once();

        let first_render = ctx.create_signal(true);
        ctx.create_effect(|| {
            if !*first_render.get() {
                toggle_cards();
            }
            first_render.set(false);
            println!("{}", first_render.get());
        });

        view! {ctx, 
        div(class="wrapper") {
            h1(class="text-align-center") { "RemembeRS" }
            div(id="game") {
                div(class="card") {
                    "Hello, world!"
                }
            }
        }
    }});
}

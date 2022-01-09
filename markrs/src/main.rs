mod local_storage;

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

macro_rules! wasm_import {
    ($name:ident()) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $name();
        }
    };
    ($name:ident( $( $arg:ident: $type:ty ),* )) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $name($($arg: $type),*);
        }
    };
    ($name:ident($($arg:ident: $type:ty),*) > $ret:ty) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $name($($arg: $type),*) -> $ret;
        }
    };
    ($name:ident() > $ret:ty) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $name() -> $ret;
        }
    }
}

wasm_import!(setInterval(closure: &Closure<dyn FnMut()>, ms: u32) > f64);
wasm_import!(clearInterval(id: f64));

fn main() {
    let value = Signal::new(local_storage::get_item("markdown").unwrap_or_default());
    let rendered = Signal::new(String::new());

    let cb = cloned!((value, rendered) => Closure::wrap(Box::new(move || {
        let inp = &*value.get();
        local_storage::set_item("markdown", inp);
        rendered.set(markdown::to_html(inp));
    }) as Box<dyn FnMut()>));

    let id = setInterval(&cb, 1000);

    cb.forget();

    on_cleanup(move || {
        clearInterval(id);
    });

    sycamore::render(|| {
        view! {
            h1(class="text-align-center") { "MarkRS" }
            div(class="wrapper") {
                    div(class="flex-container-row") {
                        div(class="flex-container-column flex-child") {
                            textarea(bind:value=value)
                        }
                        div(class="card flex-child", dangerously_set_inner_html=&*rendered.get())
                    }
            }
            div(class="footer") {
                "Made with Rust 1.57.0 and WASM"
            }
        }
    });
}

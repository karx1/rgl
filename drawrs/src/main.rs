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
    ($name:ident($($arg:ident: $type:ty),* > $ret:ty)) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $name($($arg: $type)*) -> $ret;
        }
    }
}

macro_rules! read_js_value {
    ($target:expr, $key:expr) => {js_sys::Reflect::get(&$target, &wasm_bindgen::JsValue::from_str($key))}
}

wasm_import!(test(s: &str));
wasm_import!(addEventListener(name: &str, cb: &Closure<dyn Fn(MouseEvent)>));


#[wasm_bindgen]
extern "C" {
    pub type MouseEvent;
}


fn main() {
    let clicked = Signal::new(false);
    let cb = Closure::wrap(Box::new(cloned!((clicked) => move |e: MouseEvent| {
        let val = read_js_value!(e.obj, "buttons").unwrap().as_f64().unwrap() as u8;

        if val == 1 {
            clicked.set(true);
        }
    })) as Box<dyn Fn(MouseEvent)>);

    addEventListener("mousedown", &cb);

    cb.forget();

    sycamore::render(||
        template! {
            div(class="wrapper") {
                canvas(id="canvas")
            }
        }
    );
}

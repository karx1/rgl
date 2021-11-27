#![allow(non_snake_case)]

use std::panic;
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
            pub fn $name($($arg: $type)*) -> $ret;
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

macro_rules! wasm_import_type {
    ($name:ident) => {
        #[wasm_bindgen]
        extern "C" {
            pub type $name;
        }
    }
}

macro_rules! read_js_value {
    ($target:expr, $key:expr) => {
        js_sys::Reflect::get(&$target, &wasm_bindgen::JsValue::from_str($key))
    };
}

wasm_import_type!(MouseEvent);
wasm_import_type!(DOMRect);
wasm_import_type!(HTMLCanvasElement);

wasm_import!(test(s: &str));
wasm_import!(addEventListener(
    name: &str,
    cb: &Closure<dyn Fn(MouseEvent)>
));
wasm_import!(getClientRect() > DOMRect);
wasm_import!(draw(x0: f64, y0: f64, x1: f64, y1: f64));
wasm_import!(getWidth() > f64);
wasm_import!(getHeight() > f64);
wasm_import!(clear());

#[derive(Clone, Copy, Debug)]
struct Rect {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl Rect {
    fn new(rect: DOMRect) -> Self {
        let left = read_js_value!(rect.obj, "left").unwrap().as_f64().unwrap();
        let right = read_js_value!(rect.obj, "right").unwrap().as_f64().unwrap();
        let top = read_js_value!(rect.obj, "top").unwrap().as_f64().unwrap();
        let bottom = read_js_value!(rect.obj, "bottom")
            .unwrap()
            .as_f64()
            .unwrap();

        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: f64,
    y: f64,
}

fn get_mouse_pos(rect: DOMRect, evt: MouseEvent) -> Pos {
    let rust_rect = Rect::new(rect);

    let clientX = read_js_value!(evt.obj, "clientX")
        .unwrap()
        .as_f64()
        .unwrap();
    let clientY = read_js_value!(evt.obj, "clientY")
        .unwrap()
        .as_f64()
        .unwrap();

    let width = getWidth();
    let height = getHeight();

    Pos {
        x: (clientX - rust_rect.left) / (rust_rect.right - rust_rect.left) * width,
        y: (clientY - rust_rect.top) / (rust_rect.bottom - rust_rect.top) * height,
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let clicked = Signal::new(false);
    let prevPos = Signal::new(Pos { x: 0f64, y: 0f64 });

    let cb0 = Closure::wrap(Box::new(cloned!((clicked) => move |e: MouseEvent| {
        let val = read_js_value!(e.obj, "buttons").unwrap().as_f64().unwrap() as u8;

        clicked.set(val == 1);
    })) as Box<dyn Fn(MouseEvent)>);

    let cb1 = Closure::wrap(Box::new(cloned!((clicked) => move |e: MouseEvent| {
        let prev = *prevPos.get();
        let pos = get_mouse_pos(getClientRect(), e);
        if *clicked.get() {
            draw(prev.x, prev.y, pos.x, pos.y);
        }
        prevPos.set(pos);
    })) as Box<dyn Fn(MouseEvent)>);

    let cb2 = Closure::wrap(Box::new(cloned!((clicked) => move |_| {
        clicked.set(false);
    })) as Box<dyn Fn(MouseEvent)>);

    addEventListener("mousedown", &cb0);
    addEventListener("mousemove", &cb1);
    addEventListener("mouseup", &cb2);

    cb0.forget();
    cb1.forget();
    cb2.forget();

    sycamore::render(|| {
        template! {
            div(class="wrapper") {
                h1(class="text-align-center") { "DrawRS" }
                div(class="text-align-center") {
                    button(on:click=|_| clear()) { "Clear" }
                    br
                    button(class="color-button", id="red")
                    button(class="color-button", id="orange")
                    button(class="color-button", id="yellow")
                    button(class="color-button", id="green")
                    button(class="color-button", id="blue")
                    button(class="color-button", id="indigo")
                    button(class="color-button", id="red")
                }
                canvas(id="canvas")
            }
        }
    });
}

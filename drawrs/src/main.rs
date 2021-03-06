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
    };
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
wasm_import!(draw(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    color: &str,
    width: u8
));
wasm_import!(getWidth() > f64);
wasm_import!(getHeight() > f64);
wasm_import!(clear());
wasm_import!(updateDownload());

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
    let color = Signal::new(String::from("black"));
    let size = Signal::new(String::from("small"));

    let cb0 = Closure::wrap(Box::new(cloned!((clicked) => move |e: MouseEvent| {
        let val = read_js_value!(e.obj, "buttons").unwrap().as_f64().unwrap() as u8;

        clicked.set(val == 1);
    })) as Box<dyn Fn(MouseEvent)>);

    let cb1 = Closure::wrap(
        Box::new(cloned!((clicked, color, size) => move |e: MouseEvent| {
            let prev = *prevPos.get();
            let pos = get_mouse_pos(getClientRect(), e);
            if *clicked.get() {
                let width = match &**size.get() {
                    "small" => 4,
                    "med" => 8,
                    "large" => 16,
                    _ => 4, // Set default to 4 just in case
                };
                draw(prev.x, prev.y, pos.x, pos.y, &*color.get(), width);
                updateDownload();
            }
            prevPos.set(pos);
        })) as Box<dyn Fn(MouseEvent)>,
    );

    let cb2 = Closure::wrap(Box::new(cloned!((clicked) => move |_| {
        clicked.set(false);
    })) as Box<dyn Fn(MouseEvent)>);

    addEventListener("mousedown", &cb0);
    addEventListener("mousemove", &cb1);
    addEventListener("mouseup", &cb2);

    cb0.forget();
    cb1.forget();
    cb2.forget();

    let click_color_div = cloned!((color) => move |e: web_sys::Event| {
        let id = read_js_value!(e.target().unwrap(), "id").unwrap().as_string().unwrap();

        color.set(id);
    });

    let click_size_button = cloned!((size) => move |e: web_sys::Event| {
        let id = read_js_value!(e.target().unwrap(), "id").unwrap().as_string().unwrap();

        size.set(id);
    });

    let click_clear_button = |_| {
        clear();
        updateDownload();
    };

    sycamore::render(|| {
        template! {
            div(class="wrapper") {
                h1(class="text-align-center") { "DrawRS" }
                div(class="text-align-center") {
                    a(id="download", class="button") { "Save" }
                    button(on:click=click_clear_button, id="clear") { "Clear" }
                    br
                    button(class="size-button", id="small", on:click=click_size_button.clone()) { "Small" }
                    button(class="size-button", id="med", on:click=click_size_button.clone()) { "Medium" }
                    button(class="size-button", id="large", on:click=click_size_button.clone()) { "Large" }
                    br
                    div(on:click=click_color_div.clone(), class="color-button", id="red")
                    div(on:click=click_color_div.clone(), class="color-button", id="orange")
                    div(on:click=click_color_div.clone(), class="color-button", id="yellow")
                    div(on:click=click_color_div.clone(), class="color-button", id="green")
                    div(on:click=click_color_div.clone(), class="color-button", id="blue")
                    div(on:click=click_color_div.clone(), class="color-button", id="indigo")
                    div(on:click=click_color_div.clone(), class="color-button", id="violet")
                    div(on:click=click_color_div.clone(), class="color-button", id="brown")
                    div(on:click=click_color_div.clone(), class="color-button", id="black")
                    div(on:click=click_color_div.clone(), class="color-button", id="white")
                }
                canvas(id="canvas")
            }
        }
    });
}

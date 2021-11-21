mod interval;

use sycamore::prelude::*;
use wasm_bindgen::closure::Closure;
use crate::interval::set_interval;

fn main() {
    let time_left = Signal::new(60u8);

    let cb = Closure::wrap(Box::new(cloned!((time_left) => move || {
        let time = *time_left.get();
        time_left.set(time - 1);
    })) as Box<dyn Fn()>);

    set_interval(&cb, 1_000);

    cb.forget();

    sycamore::render(|| template! {
        div(class="wrapper") {
            p { (time_left.get()) }
        }
    });
}

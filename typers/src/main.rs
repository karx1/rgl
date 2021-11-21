mod interval;

use crate::interval::set_interval;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = processText)]
    fn process_text(value: String) -> usize;
}

fn main() {
    let time_left = Signal::new(60u8);
    let value = Signal::new(String::new());
    let error_count = Signal::new(0usize);
    let current_quote = Signal::new(
        String::from("The quick brown fox jumps over the lazy dog")
            .chars()
            .collect::<Vec<char>>(),
    );

    let cb = Closure::wrap(Box::new(cloned!((time_left) => move || {
        let time = *time_left.get();
        if time > 0 {
            time_left.set(time - 1);
        }
    })) as Box<dyn Fn()>);

    set_interval(&cb, 1_000);

    cb.forget(); // This leaks memory but without it the closure is dropped before it can be called by the interval

    create_effect(cloned!((value, error_count) => move || {
        let value = (*value.get()).clone();

        if value.len() > 0 {
            let errors = process_text(value);
            error_count.set(errors);
        }
    }));

    sycamore::render(|| {
        template! {
            div(class="wrapper") {
                p { (time_left.get()) }
                p { (error_count.get()) }
                div(id="quote") {
                    Keyed(KeyedProps {
                        iterable: current_quote.handle(),
                        template: |c| template! {
                            span { (c) }
                        },
                        key: |c| *c
                    })
                }
                input(bind:value=value)
            }
        }
    });
}

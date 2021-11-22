mod interval;

use crate::interval::set_interval;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = processText)]
    fn process_text(value: String) -> usize;
    #[wasm_bindgen]
    fn reset();
}

fn main() {
    let time_left = Signal::new(60u8);
    let value = Signal::new(String::new());
    let error_count = Signal::new(0usize);
    let total_errors = Signal::new(0u64);
    let current_quote = Signal::new(
        String::from("Push yourself, because no one else is going to do it for you.")
            .chars()
            .collect::<Vec<char>>(),
    );
    let initial = Signal::new(true);

    let cb = Closure::wrap(Box::new(cloned!((time_left) => move || {
        let time = *time_left.get();
        if time > 0 {
            time_left.set(time - 1);
        }
    })) as Box<dyn Fn()>);

    set_interval(&cb, 1_000);

    cb.forget(); // This leaks memory but without it the closure is dropped before it can be called by the interval

    let quotes_array = [
        "Push yourself, because no one else is going to do it for you.",
        "Failure is the condiment that gives success its flavor.",
        "Wake up with determination. Go to bed with satisfaction.",
        "It's going to be hard, but hard does not mean impossible.",
        "Learning never exhausts the mind.",
        "The only way to do great work is to love what you do.",
    ];

    let try_update_quote = cloned!((current_quote, quotes_array, value, total_errors, error_count) => move |inp_value: &str| {
        let quote = (*current_quote.get()).iter().cloned().collect::<String>();
        if inp_value.len() == quote.len() {
            let index = quotes_array.iter().position(|&c| c.to_string() == quote).unwrap(); // if it's None, something's gone very wrong with time

            if index == (quotes_array.len() - 1) {
                current_quote.set(String::from(quotes_array[0]).chars().collect());
            } else {
                current_quote.set(String::from(quotes_array[index + 1]).chars().collect());
            }

            let t_error_count = *total_errors.get();
            let c_error_count = *error_count.get();

            total_errors.set(t_error_count + c_error_count as u64);

            error_count.set(0);

            value.set(String::new());

            reset();
        }
    });

    create_effect(cloned!((value, error_count) => move || {
        let value = (*value.get()).clone();

        if !*initial.get_untracked() { // prevent running on initial render
            let errors = process_text(value.clone());
            error_count.set(errors);

            try_update_quote(&value);

        }
        initial.set(false);
    }));

    sycamore::render(|| {
        template! {
            div(class="wrapper") {
                p { (time_left.get()) }
                p { (total_errors.get()) }
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

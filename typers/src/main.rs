mod interval;

use crate::interval::{clear_interval, set_interval};
use sycamore::{prelude::*, rt::Event};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = processText)]
    fn process_text(value: String) -> usize;
    #[wasm_bindgen]
    fn reset();
    #[wasm_bindgen]
    fn reload(event: Event);
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
    let characters_typed = Signal::new(0u64);
    let cpm = Signal::new(0f64);
    let wpm = Signal::new(0f64);
    let finished = Signal::new(false);
    let id = Signal::new(0i32);

    let cb = Closure::wrap(Box::new(cloned!((time_left) => move || {
        let time = *time_left.get();
        if time > 0 {
            time_left.set(time - 1);
        }
    })) as Box<dyn Fn()>);

    id.set(set_interval(&cb, 1_000));

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

    create_effect(cloned!((value, error_count, characters_typed) => move || {
        let value = (*value.get()).clone();

        if !*initial.get_untracked() { // prevent running on initial render
            let chars = *characters_typed.get_untracked();
            characters_typed.set(chars + 1);
            let errors = process_text(value.clone());
            error_count.set(errors);

            try_update_quote(&value);

        }
        initial.set(false);
    }));

    create_effect(cloned!((characters_typed, cpm, wpm, time_left) => move || {
        let time_elapsed = 60 - *time_left.get();
        let chars = *characters_typed.get();
        if time_elapsed > 0 {
            {
                let result = chars as f64 / time_elapsed as f64;
                cpm.set(result * 60f64);
            };

            {
                let mut result = chars as f64 / 5f64;
                result /= time_elapsed as f64;
                wpm.set(result * 60f64);
            }
        }
    }));

    create_effect(cloned!((id, time_left, finished) => move || {
        let time = *time_left.get();
        if time == 0 {
            clear_interval(*id.get_untracked()); // id should never change anyway, but in case it does it's best not to track it
            finished.set(true);
        }
    }));

    sycamore::render(|| {
        template! {
            div(class="wrapper") {
                p { (time_left.get()) }
                p { (total_errors.get()) }
                p { (error_count.get()) }
                p { (characters_typed.get()) }
                p { (cpm.get()) }
                p { (wpm.get()) }
                (if !*finished.get() {
                    cloned!((value) => template! {
                        div(id="quote") {
                            Keyed(KeyedProps {
                                iterable: current_quote.handle(),
                                template: |c| template! {
                                    span { (c) }
                                },
                                key: |c| *c
                            })
                            input(bind:value=value, class="text-align-center")
                        }
                    })
                } else {
                    template! {
                        div(class="text-align-center") {
                            "Great job!"
                            br
                            button(on:click=reload) { "Click to restart" }
                        }
                    }
                })
            }
        }
    });
}

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

#[derive(Clone, Copy, Debug)]
struct Stats {
    errors: u64,
    cpm: f64,
    wpm: f64,
}

#[derive(Clone, Debug)]
struct Props {
    mode: Signal<AppMode>,
    stats: Signal<Stats>,
}

#[component(TestComponent<G>)]
fn test_component(props: Props) -> Template<G> {
    let mode = props.mode;
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

    create_effect(
        cloned!((id, time_left, finished, total_errors, error_count) => move || {
            let time = *time_left.get();
            if time == 0 {
                clear_interval(*id.get_untracked()); // id should never change anyway, but in case it does it's best not to track it
                let t_error_count = *total_errors.get_untracked();
                let c_error_count = *error_count.get_untracked();

                total_errors.set(t_error_count + c_error_count as u64);
                finished.set(true);
            }
        }),
    );

    create_effect(
        cloned!((finished, mode, total_errors, cpm, wpm) => move || {
            if *finished.get() {
                let stats = Stats {
                    errors: *total_errors.get_untracked(),
                    cpm: *cpm.get_untracked(),
                    wpm: *wpm.get_untracked(),
                };
                props.stats.set(stats);
                mode.set(AppMode::Restart);
            }
        }),
    );

    template! {
        div(class="text-align-center") {
            div(class="inline card") {
                "Time Left"
                br {}
                (time_left.get())
            }
            div(class="inline incorrect card") {
                "Total Errors"
                br {}
                (total_errors.get())
            }
            div(class="inline card") {
                "CPM"
                br {}
                (((*cpm.get()).round() as u64))
            }
            div(class="inline card") {
                "WPM"
                br {}
                (((*wpm.get()).round() as u64))
            }
        }
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
    }
}

#[component(RestartView<G>)]
fn restart_view(props: Props) -> Template<G> {
    let mode = props.mode;
    let restart = cloned!((mode) => move |_| {
        mode.set(AppMode::Test);
    });
    let stats = *props.stats.get();
    let errors = stats.errors;
    let cpm = stats.cpm as u64;
    let wpm = stats.wpm as u64;
    template! {
        div(class="text-align-center") {
            div(class="inline incorrect card") {
                "Total Errors"
                br {}
                (errors)
            }
            div(class="inline card") {
                "CPM"
                br {}
                (cpm)
            }
            div(class="inline card") {
                "WPM"
                br {}
                (wpm)
            }
            br {}
            "Great job!"
            br
            button(on:click=restart) { "Click to restart" }
        }
    }
}

#[component(StartScreen<G>)]
fn start_screen(mode: Signal<AppMode>) -> Template<G> {
    let start = cloned!((mode) => move |_| {
        mode.set(AppMode::Test);
    });
    template! {
        div(class="text-align-center") {
            button(on:click=start) { "Start" }
        }
    }
}

#[derive(Clone, Debug)]
enum AppMode {
    Start,
    Test,
    Restart,
}

fn main() {
    let mode = Signal::new(AppMode::Start);
    let stats = Signal::new(Stats {
        errors: 0,
        cpm: 0f64,
        wpm: 0f64,
    });
    sycamore::render(|| {
        template! {
            h1(class="text-align-center") { "TypeRS" }
            div(class="wrapper") {
                (match *mode.get() {
                    AppMode::Start => template! { StartScreen(cloned!((mode) => mode)) },
                    AppMode::Test => template! { TestComponent(cloned!((mode, stats) => Props { mode, stats })) },
                    AppMode::Restart => template! { RestartView(cloned!((mode, stats) => Props { mode, stats })) },
                })
            }
            div(class="footer") {
                "Powered by Rust 1.56 and WASM"
            }
        }
    });
}

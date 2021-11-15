use crate::interval::{clear_interval, set_interval};
use crate::AppMode;
use sycamore::prelude::*;
use wasm_bindgen::closure::Closure;

#[derive(Debug, Clone)]
pub struct Props {
    pub mode: Signal<AppMode>,
    pub preload: Signal<bool>,
}

#[component(WorkingView<G>)]
pub fn working_view(props: Props) -> Template<G> {
    let time_left = Signal::new(1500);
    let id = Signal::new(0i32);
    let paused = Signal::new(!*props.preload.get());

    let cb = Closure::wrap(Box::new(cloned!((time_left, paused) => move || {
        if !*paused.get() {
            let time = *time_left.get();
            if time == 0 {
                props.preload.set(true);
                props.mode.set(AppMode::Break);
            }
            time_left.set(time - 1);
        }
    })) as Box<dyn Fn()>);

    id.set(set_interval(&cb, 1_000));

    on_cleanup(cloned!((id) => move || {
        clear_interval(*id.get());
    }));

    let play = cloned!((paused) => move |_| paused.set(false));
    let pause = cloned!((paused) => move |_| paused.set(true));
    let reset = cloned!((paused, time_left) => move |_| {
        paused.set(true);
        time_left.set(1500);
    });

    cb.forget();

    template! {
        p(class="countdown") {(format_time(*time_left.get()))}
        div(style="text-align: center") {
            button(on:click=play) { "Play" }
            button(on:click=pause) { "Pause" }
            button(on:click=reset) { "Reset" }
        }
    }
}

#[component(BreakView<G>)]
pub fn break_view(props: Props) -> Template<G> {
    let time_left = Signal::new(300);
    let id = Signal::new(0i32);
    let paused = Signal::new(!*props.preload.get());

    let cb = Closure::wrap(Box::new(cloned!((time_left, paused) => move || {
        if !*paused.get() {
            let time = *time_left.get();
            if time == 0 {
                props.preload.set(true);
                props.mode.set(AppMode::Working);
            }
            time_left.set(time - 1);
        }
    })) as Box<dyn Fn()>);

    id.set(set_interval(&cb, 1_000));

    on_cleanup(cloned!((id) => move || {
        clear_interval(*id.get());
    }));

    let play = cloned!((paused) => move |_| paused.set(false));
    let pause = cloned!((paused) => move |_| paused.set(true));
    let reset = cloned!((paused, time_left) => move |_| {
        paused.set(true);
        time_left.set(300);
    });

    cb.forget();

    template! {
        p(class="countdown") {(format_time(*time_left.get()))}
        div(style="text-align: center") {
            button(on:click=play) { "Play" }
            button(on:click=pause) { "Pause" }
            button(on:click=reset) { "Reset" }
        }
    }
}

fn format_time(seconds_left: u32) -> String {
    let minutes = (seconds_left / 60) % 60;
    let seconds = seconds_left % 60;

    format!(
        "{}:{}",
        add_leading_zeroes(minutes),
        add_leading_zeroes(seconds)
    )
}

fn add_leading_zeroes(num: u32) -> String {
    if num < 10 {
        format!("0{}", num)
    } else {
        // quick and easy string making
        format!("{}", num)
    }
}

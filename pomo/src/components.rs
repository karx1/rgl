use sycamore::prelude::*;
use crate::interval::{set_interval, clear_interval};
use wasm_bindgen::closure::Closure;

#[component(WorkingView<G>)]
pub fn working_view() -> Template<G> {
    let time_left = Signal::new(1500);
    let id = Signal::new(0i32);

    let cb = Closure::wrap(Box::new(cloned!((time_left) => move || {
        let time = *time_left.get();
        time_left.set(time - 1);
    })) as Box<dyn Fn()>);

    id.set(set_interval(&cb, 1_000));

    on_cleanup(cloned!((id) => move || {
        clear_interval(*id.get());
    }));

    cb.forget();

    template! {
        p(class="countdown") {
            (format_time(*time_left.get()))
        }
    }
}

fn format_time(seconds_left: u32) -> String {
    let minutes = (seconds_left / 60) % 60;
    let seconds = seconds_left % 60;

    format!("{}:{}", add_leading_zeroes(minutes), add_leading_zeroes(seconds))
}

fn add_leading_zeroes(num: u32) -> String {
    if num < 10 {
        format!("0{}", num)
    } else {
        // quick and easy string making
        format!("{}", num)
    }
}

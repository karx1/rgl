mod components;
mod interval;

use sycamore::prelude::*;

#[allow(unused)]
enum AppMode {
    Working,
    Break,
}

fn main() {
    let mode = Signal::new(AppMode::Working);

    let enter_working = cloned!((mode) => move |_| {
        mode.set(AppMode::Working);
    });

    let enter_break = cloned!((mode) => move |_| {
        mode.set(AppMode::Break);
    });

    sycamore::render(|| {
        template! {
            div(class="wrapper") {
                h1(style="text-align: center") { "Pomo" }
                div(style="text-align: center") {
                    button(on:click=enter_working) { "Working" }
                    button(on:click=enter_break) { "Break" }
                }
                (
                    match *mode.get() {
                        AppMode::Working => template! { components::WorkingView() },
                        AppMode::Break => template! { components::BreakView() },
                    }
                )
            }
        }
    });
}

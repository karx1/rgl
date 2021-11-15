mod components;
mod interval;

use sycamore::prelude::*;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum AppMode {
    Working,
    Break,
}

fn main() {
    let mode = Signal::new(AppMode::Working);
    let preload = Signal::new(false);

    let enter_working = cloned!((mode) => move |_| {
        mode.set(AppMode::Working);
    });

    let enter_break = cloned!((mode) => move |_| {
        mode.set(AppMode::Break);
    });

    let props = cloned!((mode, preload) => crate::components::Props {
        mode,
        preload
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
                        AppMode::Working => template! { components::WorkingView(cloned!((props) => props)) },
                        AppMode::Break => template! { components::BreakView(cloned!((props) => props))},
                    }
                )
            }
        }
    });
}

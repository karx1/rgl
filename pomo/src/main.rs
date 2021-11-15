mod components;
mod interval;

use sycamore::prelude::*;

#[allow(unused)]
enum AppMode {
    Working,
    Break
}

fn main() {
    sycamore::render(|| template! {
        div(class="wrapper") {
            h1(style="text-align: center") { "Pomo" }
            components::WorkingView()
        }
    });
}

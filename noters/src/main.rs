mod components;
mod console;
mod local_storage;

use sycamore::prelude::*;

#[allow(unused_macros)]
macro_rules! log {
    ($($t:tt)*) => (console::log_raw(&format_args!($($t)*).to_string()))
}

#[allow(unused)] // temp
pub enum AppMode {
    Default // note list view
}

fn main() {
    let mode = Signal::new(AppMode::Default);

    sycamore::render(|| template! {
        h1(style="text-align: center") { "NoteRS" }
        div(class="wrapper") {
            (match *mode.get() {
                AppMode::Default => template! {
                    crate::components::DefaultView()
                }
            })
        }
    });
}

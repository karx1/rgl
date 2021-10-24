#![allow(unreachable_patterns)] // Because we can exhaust AppModes in the `match`

mod components;
mod console;
mod date;
mod local_storage;

use sycamore::prelude::*;

#[allow(unused_macros)]
#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (crate::console::log_raw(&format_args!($($t)*).to_string()))
}

#[derive(Debug)]
#[allow(unused)] // temp
pub enum AppMode {
    Default, // note list view
    Create,  // note create view (might be merged into edit)
}

fn main() {
    use crate::components::*;
    let mode = Signal::new(AppMode::Default);
    let selected = Signal::new(String::new());

    sycamore::render(|| {
        template! {
            h1(style="text-align: center") { "NoteRS" }
            div(class="wrapper") {
                (match *mode.get() {
                    AppMode::Default => template! {
                        DefaultView(DefaultViewProps::new(cloned!((mode) => mode), cloned!((selected) => selected)))
                    },
                    AppMode::Create => template! {
                       CreateView(CreateViewProps::new(cloned!((mode) => mode), cloned!((selected) => selected.handle())))
                    },
                    _ => template! {
                        DefaultView(DefaultViewProps::new(cloned!((mode) => mode), cloned!((selected) => selected)))
                    }
                })
            }
        }
    });
}

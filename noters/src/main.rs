mod components;
mod console;
mod local_storage;

use sycamore::prelude::*;


macro_rules! log {
    ($($t:tt)*) => (console::log_raw(&format_args!($($t)*).to_string()))
}

#[allow(unused)] // temp
pub enum AppMode {
    Default // note list view
}

fn main() {
    #[allow(unused_variables)] // Allow because localStorage might be empty
    let keys = {
        let mut new_vec: Vec<String> = Vec::new();

        let keys_raw = local_storage::list_local_storage_keys().to_vec();

        for val in keys_raw {
            if val.is_string() {
                if let Some(res) = val.as_string() {
                    log!("{}", res);
                    new_vec.push(res);
                }
            }
        }

        new_vec
    };

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

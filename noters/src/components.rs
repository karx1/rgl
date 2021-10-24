use sycamore::prelude::*;

use crate::{
    date::{get_current_time_millis, time_hr},
    local_storage, log, AppMode,
};

#[derive(Clone, Debug)]
pub struct DefaultViewProps {
    mode: Signal<AppMode>,
    selected: Signal<String>,
}

impl DefaultViewProps {
    pub fn new(mode: Signal<AppMode>, selected: Signal<String>) -> Self {
        Self { mode, selected }
    }
}

fn truncate(s: String, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => format!("{}...", &s[..idx]),
    }
}

#[component(DefaultView<G>)]
pub fn default_view(props: DefaultViewProps) -> Template<G> {
    let mode = props.clone().mode;
    let selected = props.clone().selected;
    let templates = Template::new_fragment({
        let mut new_vec: Vec<Template<G>> = Vec::new();

        let keys = local_storage::list_local_storage_keys().to_vec();

        for val in keys {
            if val.is_string() {
                if let Some(res) = val.as_string() {
                    let note = local_storage::get_item(&res);
                    let trunced = truncate(note, 100);

                    new_vec.push(template! {
                        div(class="card") {
                            (trunced)
                            br
                            br
                            small {
                                (format!("Created at {}", time_hr(res.parse::<u64>().unwrap())))
                            }
                        }
                    });
                }
            }
        }

        new_vec
    });

    let start_create = cloned!((mode, selected) => move |_| {
        let timestamp = format!("{}", get_current_time_millis());

        selected.set(timestamp);
        mode.set(AppMode::Create);
    });

    template! {
        div(class="pull-right") {
            button(on:click=start_create) { "Create" }
        }
        div {
            (templates)
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateViewProps {
    mode: Signal<AppMode>,
    selected: StateHandle<String>,
}

impl CreateViewProps {
    pub fn new(mode: Signal<AppMode>, selected: StateHandle<String>) -> Self {
        Self { mode, selected }
    }
}

#[component(CreateView<G>)]
pub fn create_view(props: CreateViewProps) -> Template<G> {
    let value = Signal::new(String::new());
    let mode = props.clone().mode;
    let selected = props.clone().selected;

    let save = cloned!((mode, selected, value) => move |_| {
        let timestamp = &*selected.get(); // deref to turn it into a String, then borrow again to make a &str
        let note = &*value.get(); // deref to turn it into a String, then borrow again to make a &str

        log!("{}", timestamp);
        local_storage::set_item(timestamp, note);
        mode.set(AppMode::Default); // Return to default screen
    });

    let go_back = cloned!((mode) => move |_| {
        mode.set(AppMode::Default); // Return to default screen
    });

    template! {
        div(class="pull-left") {
            button(on:click=go_back) { "Go Back" }
        }
        div(style="display: flex; flex-direction: column; height: 75vh") {
           textarea(bind:value=value, style="resize: vertical; flex-grow: 1")
           br
           div(style="text-align: center;") {
               button(on:click=save) { "Save" }
           }
        }
    }
}

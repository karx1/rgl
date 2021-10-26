use sycamore::prelude::*;

use crate::{
    date::{self, get_current_time_millis, time_hr},
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
    // Grab the index of the char at `max_chars`
    match s.char_indices().nth(max_chars) {
        // None indicates that max_chars is higher than the length of the string. In this case, we can just return the string
        None => s,
        // Some returns the index, so then we can return a string slice from the beginning to index and add `...`
        Some((idx, _)) => format!("{}...", &s[..idx]),
    }
}

#[component(DefaultView<G>)]
pub fn default_view(props: DefaultViewProps) -> Template<G> {
    let mode = props.mode;
    let selected = props.selected;

    let templates = Template::new_fragment({
        let mut new_vec: Vec<Template<G>> = Vec::new();

        let keys = local_storage::list_local_storage_keys().to_vec();

        for val in keys {
            if val.is_string() {
                if let Some(res) = val.as_string() {
                    let note = local_storage::get_item(&res);
                    let trunced = truncate(note, 75);

                    let timestamp = format!("Created at {}", time_hr(res.parse::<u64>().unwrap()));
                    // Create new Strings to fix ownership problems
                    let detail_res = (&res[..]).to_string();
                    let delete_res = (&res[..]).to_string();

                    let start_detail = cloned!((mode, selected) => move |_| {
                        selected.set((&detail_res[..]).to_string());
                        mode.set(AppMode::Detail);
                    });

                    let start_edit = cloned!((mode, selected) => move |_| {
                        selected.set((&res[..]).to_string());
                        mode.set(AppMode::Edit);
                    });

                    let start_delete = cloned!((mode, selected) => move |_| {
                        selected.set((&delete_res[..]).to_string());
                        mode.set(AppMode::Delete);
                    });

                    new_vec.push(template! {
                        div(class="card") {
                            (trunced)
                            br
                            br
                            small {
                                (timestamp)
                            }
                            br
                            button(on:click=start_detail) { "View" }
                            button(on:click=start_edit) { "Edit" }
                            button(class="button-danger", on:click=start_delete) { "Delete" }
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
    let mode = props.mode;
    let selected = props.selected;

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

#[derive(Clone, Debug)]
pub struct EditViewProps {
    mode: Signal<AppMode>,
    selected: StateHandle<String>,
}

impl EditViewProps {
    pub fn new(mode: Signal<AppMode>, selected: StateHandle<String>) -> Self {
        Self { mode, selected }
    }
}

#[component(EditView<G>)]
pub fn edit_view(props: EditViewProps) -> Template<G> {
    let mode = props.mode;
    let selected = props.selected;

    let default = local_storage::get_item(&*selected.get()); // get the previously saved value
    let value = Signal::new(default); // and set it as the default state

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

#[derive(Clone, Debug)]
pub struct NoteDetailViewProps {
    mode: Signal<AppMode>,
    selected: StateHandle<String>,
}

impl NoteDetailViewProps {
    pub fn new(mode: Signal<AppMode>, selected: StateHandle<String>) -> Self {
        Self { mode, selected }
    }
}

#[component(NoteDetailView<G>)]
pub fn note_detail_view(props: NoteDetailViewProps) -> Template<G> {
    let mode = props.mode;
    let selected = props.selected;

    // explicitly denote type so that it doesn't make a &String
    let timestamp_raw: &str = &*selected.get();
    let timestamp = format!(
        "Created at {}",
        date::time_hr(timestamp_raw.parse::<u64>().unwrap())
    );

    let value = local_storage::get_item(timestamp_raw);

    let go_back = cloned!((mode) => move |_| {
        mode.set(AppMode::Default);
    });

    template! {
        div(class="pull-left") {
            button(on:click=go_back) { "Go Back" }
        }
        div(class="card") {
            (value)
            br
            br
            small {
                (timestamp)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeleteViewProps {
    mode: Signal<AppMode>,
    selected: StateHandle<String>,
}

impl DeleteViewProps {
    pub fn new(mode: Signal<AppMode>, selected: StateHandle<String>) -> Self {
        Self { mode, selected }
    }
}

#[component(DeleteView<G>)]
pub fn delete_view(props: DeleteViewProps) -> Template<G> {
    let mode = props.mode;
    let selected = props.selected;

    let cancel = cloned!((mode) => move |_| {
        mode.set(AppMode::Default);
    });

    let confirm = cloned!((selected, mode) => move |_| {
        local_storage::remove_item(&*selected.get());
        mode.set(AppMode::Default); // Return to home screen
    });

    template! {
        div(class="card", style="text-align: center") {
            "Are you sure you want to delete this note?"
            br
            br
            button(class="button-danger", on:click=confirm) { "Delete" }
            button(on:click=cancel) { "Cancel" }
        }
    }
}

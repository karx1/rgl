use sycamore::prelude::*;

use crate::local_storage;

#[component(DefaultView<G>)]
pub fn default_view() -> Template<G> {
    let templates = Template::new_fragment({
        let mut new_vec: Vec<Template<G>> = Vec::new();

        let keys = local_storage::list_local_storage_keys().to_vec();

        for val in keys {
            if val.is_string() {
                if let Some(res) = val.as_string() {
                    new_vec.push(template! { li { (res) } });
                }
            }
        }

        new_vec
    });

    template! {
        ul {
            (templates)
        }
    }
}


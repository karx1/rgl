mod local_storage;

use sycamore::prelude::*;

fn main() {

    let keys = {
        let mut new_vec: Vec<String> = Vec::new();

        let keys_raw = local_storage::list_local_storage_keys().to_vec();

        for val in keys_raw {
            if val.is_string() {
                if let Some(res) = val.as_string() {
                    new_vec.push(res);
                }
            }
        }

        new_vec
    };

    sycamore::render(|| template! {
        p { (format!("{:#?}", keys)) }
        p { (local_storage::get_item("state")) }
    });
}

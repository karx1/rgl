use sycamore::prelude::*;

#[component(DefaultView<G>)]
pub fn default_view() -> Template<G> {
    template! {
        p { "Hello, world!" }
    }
}


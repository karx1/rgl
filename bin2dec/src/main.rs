use sycamore::prelude::*;

fn main() {
    sycamore::render(|| {
        template! {
            p { "Hello, world!" }
        }
    })
}

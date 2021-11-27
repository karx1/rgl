use sycamore::prelude::*;

fn main() {
    sycamore::render(||
        template! {
            div(class="wrapper") {
                p { "Hello, world!" }
            }
        }
    );
}

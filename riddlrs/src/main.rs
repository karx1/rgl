use sycamore::prelude::*;

fn main() {
    sycamore::render(|| template! {
        div(class="wrapper") {
            h1(class="text-align-center") { "RiddlRS" }
            p { "Hello, world!" }
        }
    });
}

use sycamore::prelude::*;

fn main() {
    sycamore::render(|| template! {
        div(class="wrapper") {
            h1(style="text-align: center") { "Hello, World!" }
        }
    });
}

use sycamore::prelude::*;

fn main() {
    sycamore::render(|| view! {
        h1(class="text-align-center") { "MarkRS" }
        div(class="wrapper") {
            p { "Hello, world!" }
        }
        div(class="footer") {
            "Made with Rust 1.57.0 and WASM"
        }
    });
}

use sycamore::prelude::*;

fn main() {
    sycamore::render(|| {
        view! {
            h1(class="text-align-center") { "CryptRS" }
            div(class="wrapper") {
                p { "Hello, world!" }
            }
        }
    });
}

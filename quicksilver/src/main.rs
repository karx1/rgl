use sycamore::prelude::*;

fn main() {
    sycamore::render(|ctx| {
        view! {ctx,
            div(class="wrapper") {
                h1(class="text-align-center") { "Quicksilver" }
            }
        }
    });
}

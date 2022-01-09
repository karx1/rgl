use sycamore::prelude::*;

fn main() {
    let value = Signal::new(String::new());
    let rendered = create_memo(cloned!(value => move || markdown::to_html(&*value.get())));

    sycamore::render(|| {
        view! {
            h1(class="text-align-center") { "MarkRS" }
            div(class="flex-container-row wrapper") {
                div(class="flex-child") {
                    div(class="flex-container-column") {
                        textarea(bind:value=value, class="flex-child")
                        div(class="card flex-child", dangerously_set_inner_html=&*rendered.get())
                    }
                }
            }
            div(class="footer") {
                "Made with Rust 1.57.0 and WASM"
            }
        }
    });
}

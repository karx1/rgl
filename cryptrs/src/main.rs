use std::panic;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

enum AppMode {
    Encrypt,
    Decrypt,
}

#[component(EncryptionComponent<G>)]
fn encryption_component() -> View<G> {
    let keyword = Signal::new(String::new());
    let input = Signal::new(String::new());
    let no_key = Signal::new(false);

    let crypted = create_memo(cloned!((keyword, input, no_key) => move || {
        let keyword = &**keyword.get();

        let mut key = String::new();

        for c in keyword.chars().cycle() {
            if key.len() == input.get().len() {
                break;
            } else {
                key.push(c.to_ascii_uppercase());
            }
        }

        log(&format!("{}", key));

        let mut generated = String::with_capacity(input.get().len());

        for (i, c) in input.get().chars().enumerate() {
            if let Some(char) = key.chars().nth(i) {
                no_key.set(false);
                let mut x = (c.to_ascii_uppercase() as u16 + char as u16) % 26;

                x += 'A' as u16;

                generated.push(char::from_u32(x as u32).unwrap());
            } else {
                no_key.set(true);
            }
        }

        generated
    }));

    view! {
        label { "Keyword:"
            input(bind:value=keyword)
        }
        label { "Input:"
            input(bind:value=input)
        }
        (if *no_key.get() {
            view! {
                p(class="text-align-center", style="color: red") { "No keyword was provided" }
            }
        } else {
            view! {}
        })
        div(class="card") {
            (crypted.get())
        }
    }
}

#[component(DecryptionComponent<G>)]
fn decryption_component() -> View<G> {
    let keyword = Signal::new(String::new());
    let input = Signal::new(String::new());
    let no_key = Signal::new(false);

    let decrypted = create_memo(cloned!((keyword, input, no_key) => move || {
        let keyword = &**keyword.get();

        let mut key = String::new();

        for c in keyword.chars().cycle() {
            if key.len() == input.get().len() {
                break;
            } else {
                key.push(c.to_ascii_uppercase());
            }
        }

        log(&format!("{}", key));

        let mut generated = String::with_capacity(input.get().len());

        for (i, c) in input.get().chars().enumerate() {
            if let Some(char) = key.chars().nth(i) {
                no_key.set(false);
                let mut x = (c.to_ascii_uppercase() as i16 - char as i16 + 26) as u16 % 26;

                x += 'A' as u16;

                generated.push(char::from_u32(x as u32).unwrap());
            } else {
                no_key.set(true);
            }
        }

        generated
    }));

    view! {
        label { "Keyword:"
            input(bind:value=keyword)
        }
        label { "Input:"
            input(bind:value=input)
        }
        (if *no_key.get() {
            view! {
                p(class="text-align-center", style="color: red") { "No keyword was provided" }
            }
        } else {
            view! {}
        })
        div(class="card") {
            (decrypted.get())
        }
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mode = Signal::new(AppMode::Encrypt);

    let set_encrypt = cloned!(mode => move |_| mode.set(AppMode::Encrypt));
    let set_decrypt = cloned!(mode => move |_| mode.set(AppMode::Decrypt));

    sycamore::render(|| {
        view! {
            h1(class="text-align-center") { "CryptRS" }
            div(class="text-align-center") {
                button(on:click=set_encrypt) { "Encrypt" }
                button(on:click=set_decrypt) { "Decrypt" }
            }
            div(class="wrapper") {
                (match *mode.get() {
                    AppMode::Encrypt => view! { EncryptionComponent() },
                    AppMode::Decrypt => view! { DecryptionComponent() }
                })
            }
        }
    });
}

use sycamore::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

enum AppMode {
    Encrypt,
    Decrypt
}

#[component(EncryptionComponent<G>)]
fn encryption_component() -> View<G> {
    let keyword = Signal::new(String::new());
    let input = Signal::new(String::new());



    let crypted = create_memo(cloned!((keyword, input) => move || {
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
            let char = key.chars().nth(i).unwrap(); // we can safely assume that this is not None because we made the key the same length as input

            let mut x = (c.to_ascii_uppercase() as u16 + char as u16) % 26;

            x += 'A' as u16;

            generated.push(char::from_u32(x as u32).unwrap());
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
        div(class="card") {
            (crypted.get())
        }
    }
}

#[component(DecryptionComponent<G>)]
fn decryption_component() -> View<G> {
    let keyword = Signal::new(String::new());
    let input = Signal::new(String::new());
    
    let decrypted = create_memo(cloned!((keyword, input) => move || {
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
            let char = key.chars().nth(i).unwrap(); // we can safely assume that this is not None because we made the key the same length as input

            let mut x = (c.to_ascii_uppercase() as i16 - char as i16 + 26) as u16 % 26;

            x += 'A' as u16;

            generated.push(char::from_u32(x as u32).unwrap());
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
        div(class="card") {
            (decrypted.get())
        }
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mode = Signal::new(AppMode::Decrypt);

    sycamore::render(|| {
        view! {
            h1(class="text-align-center") { "CryptRS" }
            div(class="text-align-center") {
                button { "Encrypt" }
                button { "Decrypt" }
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
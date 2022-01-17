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
    panic::set_hook(Box::new(console_error_panic_hook::hook));
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
            input(placeholder="key", bind:value=keyword)
        }
        label { "Input:"
            input(placeholder="secrets", bind:value=input)
        }
        div(class="card") {
            (crypted.get())
        }
    }
}

fn main() {
    let mode = Signal::new(AppMode::Encrypt);

    sycamore::render(|| {
        view! {
            h1(class="text-align-center") { "CryptRS" }
            div(class="wrapper") {
                (match *mode.get() {
                    AppMode::Encrypt => view! { EncryptionComponent() },
                    AppMode::Decrypt => todo!()
                })
            }
        }
    });
}
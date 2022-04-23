use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

macro_rules! wasm_import {
    ($($tt:tt)*) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $($tt)*;
        }
    };
}
macro_rules! wasm_import_with_ns {
    ($ns: ident, $($tt:tt)*) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = $ns)]
            pub fn $($tt)*;
        }
    };
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Deck(Vec<Card>);

impl Deref for Deck {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
struct Card {
    front: String,
    back: String,
}

wasm_import!(get_token() -> Option<String>);
wasm_import_with_ns!(console, log(s: &str));

#[component]
fn CardsComponent<G: Html>(ctx: Scope) -> View<G> {
    let token = get_token().unwrap();
    let data = String::from_utf8(base64::decode(&token).unwrap()).unwrap();
    let deck: Deck = serde_json::from_str(&data).unwrap();
    let current = create_signal(ctx, rand::thread_rng().gen_range(0..deck.len()));
    log(&format!("{:#?}", *deck));
    let deck_len = deck.len();

    let recompute_current = move |_| {
        current.set(rand::thread_rng().gen_range(0..deck_len));
    };

    view! {ctx,
        button(on:click=recompute_current) { "Next" }
        ({
            let current_card = deck[*current.get()].clone();
            view! {ctx,
                div {
                    (current_card.front)
                }
                div {
                    (current_card.back)
                }
            }
        })
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|ctx| {
        view! {ctx,
            div(class="wrapper") {
                h1(class="text-align-center") { "Quicksilver" }
                (if get_token().is_some() {
                    view! {ctx, CardsComponent {

                    }}
                } else {
                    view! {ctx, }
                })
            }
        }
    });
}

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
    question: String,
    answers: [String; 4],
    correct: usize,
}

wasm_import!(get_token() -> String);
wasm_import_with_ns!(console, log(s: &str));

fn main() {
    sycamore::render(|ctx| {
        let token = get_token();
        let data = String::from_utf8(base64::decode(&token).unwrap()).unwrap();
        log(&data);
        let deck: Deck = serde_json::from_str(&data).unwrap();
        let current = create_signal(ctx, rand::thread_rng().gen_range(0..deck.len()));
        view! {ctx,
            div(class="wrapper") {
                h1(class="text-align-center") { "Quicksilver" }
            }
        }
    });
}

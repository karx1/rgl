use sycamore::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {{
        let mut map = HashMap::new();

        $(
            map.insert($key, $value);            
        )*

        map
    }}
}

macro_rules! wasm_import {
    ($name:ident()) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $name();
        }
    };
    ($name:ident( $( $arg:ident: $type:ty ),* )) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $name($($arg: $type),*);
        }
    };
    ($name:ident($($arg:ident: $type:ty),*) > $ret:ty) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $name($($arg: $type)*) -> $ret;
        }
    };
    ($name:ident() > $ret:ty) => {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            pub fn $name() -> $ret;
        }
    }
}

#[derive(Clone, Debug)]
struct Question {
    prompt: &'static str,
    answer: char,
    choices: HashMap<char, &'static str>
}

macro_rules! create_question {
    ($prompt:expr, $answer:expr, $choices:expr) => {
        Question {
            prompt: $prompt,
            answer: $answer,
            choices: $choices
        }
    }
}

wasm_import!(log(s: String));

fn main() {
    let question = create_question!("Which of these fruits contains potassium?", 'B',
        hashmap! {
            'A' => "apple",
            'B' => "banana",
            'C' => "cherry",
            'D' => "dragonfruit"
        }
    );
    log(format!("{:#?}", question));
    sycamore::render(|| template! {
        div(class="wrapper") {
            h1(class="text-align-center") { "RiddlRS" }
            p { "Hello, world!" }
        }
    });
}

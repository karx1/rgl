use sycamore::prelude::*;
use indexmap::IndexMap;
use wasm_bindgen::prelude::*;

macro_rules! indexmap {
    ($($key:expr => $value:expr),*) => {{
        let mut map = IndexMap::new();

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
    choices: IndexMap<char, &'static str>
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
    let questions = [create_question!("Which of these fruits contains potassium?", 'B',
        indexmap! {
            'A' => "apple",
            'B' => "banana",
            'C' => "cherry",
            'D' => "dragonfruit"
        }
    ),
    create_question!("The moon's light actually comes from the sun", 'A', indexmap! {
        'A' => "True",
        'B' => "False"
    }),
    create_question!("Which of these countries is not considered a kingdom?", 'C', indexmap! {
        'A' => "Belgium",
        'B' => "Denmark",
        'C' => "Monaco",
        'D' => "Sweden"
    }),
    create_question!("Force is measured in which unit?", 'B', indexmap! {
        'A' => "Kilograms",
        'B' => "Newtons",
        'C' => "Joules"
    }),
    create_question!("What does a vulcanologist study?", 'A', indexmap! {
        'A' => "Volcanoes",
        'B' => "Plants",
        'C' => "Constellations"
    }),
    create_question!("Energy is measured in which unit?", 'C', indexmap! {
        'A' => "Kilograms",
        'B' => "Newtons",
        'C' => "Joules"
    }),
    ];
    log(format!("{:#?}", questions[0]));
    sycamore::render(|| template! {
        div(class="wrapper") {
            h1(class="text-align-center") { "RiddlRS" }
            p { "Hello, world!" }
        }
    });
}

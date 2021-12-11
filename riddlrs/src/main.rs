#![allow(unused_macros)]

use indexmap::IndexMap;
use std::panic;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::Event;

macro_rules! indexmap {
    ($($key:expr => $value:expr),*) => {{
        let mut map = IndexMap::with_capacity(5);

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

macro_rules! wasm_import_type {
    ($name:ident) => {
        #[wasm_bindgen]
        extern "C" {
            pub type $name;
        }
    };
    ($($name:ident),*) => {
        #[wasm_bindgen]
        extern "C" {
            $(pub type $name;)*
        }
    }
}

macro_rules! read_js_value {
    ($target:expr, $key:expr) => {
        js_sys::Reflect::get(&$target, &wasm_bindgen::JsValue::from_str($key))
    };
}

#[derive(Clone, Debug, Default)]
struct Question {
    prompt: &'static str,
    answer: char,
    choices: IndexMap<char, &'static str>,
}

macro_rules! create_question {
    ($prompt:expr, $answer:expr, $choices:expr) => {
        Question {
            prompt: $prompt,
            answer: $answer,
            choices: $choices,
        }
    };
}

wasm_import!(log(s: String));

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let questions = [
        create_question!(
            "Which of these fruits contains potassium?",
            'B',
            indexmap! {
                'A' => "apple",
                'B' => "banana",
                'C' => "cherry",
                'D' => "dragonfruit"
            }
        ),
        create_question!(
            "The moon's light actually comes from the sun",
            'A',
            indexmap! {
                'A' => "True",
                'B' => "False"
            }
        ),
        create_question!(
            "Which of these countries is not considered a kingdom?",
            'C',
            indexmap! {
                'A' => "Belgium",
                'B' => "Denmark",
                'C' => "Monaco",
                'D' => "Sweden"
            }
        ),
        create_question!(
            "Force is measured in which unit?",
            'B',
            indexmap! {
                'A' => "Kilograms",
                'B' => "Newtons",
                'C' => "Joules"
            }
        ),
        create_question!(
            "What does a vulcanologist study?",
            'A',
            indexmap! {
                'A' => "Volcanoes",
                'B' => "Plants",
                'C' => "Constellations"
            }
        ),
        create_question!(
            "Energy is measured in which unit?",
            'C',
            indexmap! {
                'A' => "Kilograms",
                'B' => "Newtons",
                'C' => "Joules"
            }
        ),
    ];
    let index = Signal::new(0usize);
    let current_prompt = create_memo(cloned!((index, questions) => move || {
        questions[*index.get()].prompt.to_string()
    }));
    let current_choices = create_memo(cloned!((index, questions) => move || {
        questions[*index.get()].choices.clone().into_iter().collect::<Vec<(char, &str)>>()
    }));

    let answer_question = cloned!((index, questions) => move |e: Event| {
        let answer = questions[*index.get()].answer;
        let id = read_js_value!(e.target().unwrap(), "id").unwrap().as_string().unwrap().chars().collect::<Vec<char>>()[0];
        log(format!("{}", id == answer));
    });

    sycamore::render(cloned!(answer_question => || view! {
        div(class="wrapper") {
            h1(class="text-align-center") { "RiddlRS" }
            div(class="card") {
                p(class="text-align-center") { (current_prompt.get()) }
                div(class="text-align-center") {
                    Keyed(KeyedProps {
                        iterable: current_choices,
                        template: move |(c, s)| view! {
                            button(class="fw", id=c, on:click=answer_question.clone()) { (c)") "(s) }
                        },
                        key: |(c, _)| *c
                    })
                }
            }
        }
    }));
}

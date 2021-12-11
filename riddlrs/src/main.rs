#![allow(unused_macros)]

use indexmap::IndexMap;
use std::panic;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::Event;

#[derive(Clone, Debug, Default)]
struct Question {
    prompt: &'static str,
    answer: char,
    choices: IndexMap<char, &'static str>,
}

#[derive(Clone, Debug)]
enum AppMode {
    Startscreen,
    Quiz,
    Endgame,
}

#[derive(Clone, Debug)]
struct Props {
    mode: Signal<AppMode>,
    errors: Signal<usize>,
    time_elapsed: Signal<usize>,
}

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
            pub fn $name($($arg: $type),*) -> $ret;
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
wasm_import!(setInterval(closure: &Closure<dyn FnMut()>, ms: u32) > f64);
wasm_import!(clearInterval(id: f64));

#[component(QuizComponent<G>)]
fn quiz_component(props: Props) -> View<G> {
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

    let correct = Signal::new(true);
    let errors = props.errors;
    let mode = props.mode;
    let time_elapsed = props.time_elapsed;

    let answer_question = cloned!((index, questions, correct, errors) => move |e: Event| {
        let answer = questions[*index.get()].answer;
        let id = read_js_value!(e.target().unwrap(), "id").unwrap().as_string().unwrap().chars().collect::<Vec<char>>()[0];
        if answer == id {
            let current_index = *index.get();
            if current_index == 5 {
                mode.set(AppMode::Endgame);
            } else {
                index.set(*index.get() + 1);
                correct.set(true);
            }
        } else {
            errors.set(*errors.get() + 1);
            correct.set(false);
        }
    });

    create_effect(cloned!(time_elapsed => move || {
        log(format!("{}", time_elapsed.get()));
    }));

    let cb = Closure::wrap(Box::new(move || {
        let current_time = *time_elapsed.get();
        time_elapsed.set(current_time + 1);
    }) as Box<dyn FnMut()>);

    let id = setInterval(&cb, 1000);

    on_cleanup(move || {
        clearInterval(id);
        cb.forget();
    });

    view! {
            div(class="card") {
                p(class="text-align-center") { (current_prompt.get()) }
                div(class="text-align-center") {
                    Keyed(KeyedProps {
                        iterable: current_choices,
                        template: move |(c, s)| view! {
                            button(class="fw", id=c, on:click=answer_question.clone()) { (c)") "(s) }
                        },
                        key: |(_, s)| *s
                    })
                }
            }
            (if !*correct.get() {
                view! {
                    p(class="text-align-center", style="color: red") { "Incorrect. Please try again." }
                }
            } else {
                view! {}
            })
    }
}

#[component(EndGameComponent<G>)]
fn end_game_component(props: Props) -> View<G> {
    let mode = props.mode;
    let errors = props.errors;
    let time_elapsed = props.time_elapsed;

    let (failed, grade) = cloned!(errors => {
        let num_errors = (*errors.get()) as f64;

        let mut percentage = num_errors / 6f64;
        percentage *= 100f64;

        (percentage > 70f64, format!("{:.2}%", 100f64 - percentage))
    });

    let time = format!(
        "{}:{}",
        add_leading_zeroes(*time_elapsed.get() / 60),
        add_leading_zeroes(*time_elapsed.get() % 60)
    );

    let restart = cloned!((mode, errors) => move |_| {
        errors.set(0);
        time_elapsed.set(0);
        mode.set(AppMode::Quiz);
    });

    view! {
        (if failed {
            view! {
                p(class="text-align-center") { "Sorry, you failed. Better luck next time! Here's how you did:" }
            }
        } else {
            view! {
                p(class="text-align-center") { "Great job, you passed! Here's how you did:" }
            }
        })
        br
        div(class="text-align-center") {
            div(class="card text-align-center inline") {
                p {"Time taken:"}
                p { (time) }
            }
            div(class="card text-align-center inline", style="color: red") {
                p {"Errors:"}
                p { (errors.get()) }
            }
            div(class="card text-align-center inline") {
                p {"Grade:"}
                p { (grade) }
            }
            br
            button(on:click=restart) { "Restart" }
        }
    }
}

#[component(StartScreen<G>)]
fn start_screen_component(props: Props) -> View<G> {
    let mode = props.mode;

    let start_quiz = cloned!(mode => move |_| {
        mode.set(AppMode::Quiz);
    });

    view! {
        div(class="text-align-center") {
            button(on:click=start_quiz) { "Start Quiz" }
        }
    }
}

fn add_leading_zeroes(num: usize) -> String {
    if num < 10 {
        return format!("0{}", num);
    }
    return format!("{}", num);
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mode = Signal::new(AppMode::Startscreen);
    let errors = Signal::new(0usize);
    let time_elapsed = Signal::new(0usize);

    create_effect(cloned!(errors => move || {
        log(format!("{}", errors.get()));
    }));

    sycamore::render(|| {
        view! {
            div(class="wrapper") {
                h1(class="text-align-center") { "RiddlRS" }
                (match *mode.get() {
                    AppMode::Quiz => view! { QuizComponent(Props { mode: cloned!(mode => mode), errors: cloned!(errors => errors), time_elapsed: cloned!(time_elapsed => time_elapsed) }) },
                    AppMode::Endgame => view! { EndGameComponent(Props { mode: cloned!(mode => mode), errors: cloned!(errors => errors), time_elapsed: cloned!(time_elapsed => time_elapsed) }) },
                    AppMode::Startscreen => view! { StartScreen(Props { mode: cloned!(mode => mode), errors: cloned!(errors => errors), time_elapsed: cloned!(time_elapsed => time_elapsed) }) }
                })
            }
        }
    });
}

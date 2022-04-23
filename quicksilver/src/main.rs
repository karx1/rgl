use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event};

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
wasm_import!(prompt(s: &str) -> Option<String>);
wasm_import!(set_location(l: &str));
wasm_import!(alert(s: &str));
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
        let prev = *current.get();
        let mut genned = rand::thread_rng().gen_range(0..deck_len);
        if deck_len > 1 {
            while genned == prev {
                genned = rand::thread_rng().gen_range(0..deck_len);
            }
        }

        current.set(genned);
    };

    let on_click = |event: Event| {
        let elem = event
            .current_target()
            .unwrap()
            .dyn_ref::<Element>()
            .unwrap()
            .clone();

        elem.class_list().toggle("flip").unwrap();
    };

    let go_home = |_| set_location("/");

    view! {ctx,
        div(class="text-align-center") {
            button(on:click=go_home) { "Home" }
            button(on:click=recompute_current) { "Next" }
        }
        ({
            let current_card = deck[*current.get()].clone();
            view! {ctx,
                div(class="card-container") {
                        h2(class="front-face") {
                            (current_card.front)
                        }
                    div(class="card", on:click=on_click) {
                        div(class="back-face") {
                            (current_card.back)
                        }
                    }
                }
            }
        })
    }
}

#[component]
fn CreatorComponent<G: Html>(ctx: Scope) -> View<G> {
    let do_import = |_| {
        let p = prompt("Deck code:");
        if let Some(inp) = p {
            let stripped = inp
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();
            let f = format!("/?deck={}", stripped);
            set_location(&f);
        }
    };

    let front = create_signal(ctx, String::new());
    let back = create_signal(ctx, String::new());

    let error_empty = create_signal(ctx, false);
    let error_parse = create_signal(ctx, false);

    let cards = create_signal(ctx, Vec::new());

    let do_add = |_| {
        let f = (*front.get()).clone();
        let b = (*back.get()).clone();

        if f.is_empty() || b.is_empty() {
            error_empty.set(true);
            return;
        }

        let c = Card { front: f, back: b };
        if !cards.get().contains(&c) {
            cards.modify().push(c);
            front.set(String::new());
            back.set(String::new());
        } // skip duplicate cards
    };

    let do_export = |_| {
        let d = Deck((*cards.get()).clone());

        if d.is_empty() {
            return;
        }

        let r = serde_json::to_string(&d);

        if let Ok(s) = r {
            error_parse.set(false);
            let e = base64::encode(s.as_bytes());
            let f = format!("Your deck code is: {}", e);
            alert(&f);
        } else {
            error_parse.set(true);
        }
    };

    view! {ctx,
        div(class="text-align-center") {
            button(on:click=do_import) {"Import"}
            button(on:click=do_export) {"Export"}
            br
            input(bind:value=front)
            input(bind:value=back)
            (if *error_empty.get() {
                view! {ctx, p(style="color: red") {"Make sure none of the inputs are empty!"}}
            } else {
                view! {ctx,}
            })
            (if *error_parse.get() {
                view! {ctx, p(style="color: red") {"Something went wrong. Please try again."}}
            } else {
                view! {ctx,}
            })
            button(on:click=do_add) {"Add"}
            Indexed {
                iterable: cards,
                view: |ctx, card| view! {ctx,
                    div(class="card", style="background-color: white;") {
                        "Front:"
                        (card.front)
                        br
                        "Back:"
                        (card.back)
                    }
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|ctx| {
        view! {ctx,
            div(class="wrapper") {
                h1(class="text-align-center") { "Quicksilver" }
                (if get_token().is_some() {
                    view! {ctx, CardsComponent {}}
                } else {
                    view! {ctx, CreatorComponent {}}
                })
            }
        }
    });
}

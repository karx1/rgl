use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::ops::DerefMut;
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

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
struct Card {
    front: String,
    back: String,
}

wasm_import!(get_token() -> Option<String>);
wasm_import!(get_edit_token() -> Option<String>);
wasm_import!(check_history() -> bool);
wasm_import!(prompt(s: &str) -> Option<String>);
wasm_import!(set_location(l: &str));
wasm_import!(alert(s: &str));
wasm_import_with_ns!(console, log(s: &str));
wasm_import_with_ns!(localStorage, setItem(key: &str, value: &str));
wasm_import_with_ns!(localStorage, getItem(key: &str) -> Option<String>);

#[component]
fn CardsComponent<G: Html>(ctx: Scope) -> View<G> {
    let token = get_token().unwrap();
    {
        let prev = getItem("history").unwrap_or_else(|| String::from("[]"));
        let mut history: Vec<String> = serde_json::from_str(&prev).unwrap_or_default();
        history.push(token.clone());
        history.sort_unstable();
        history.dedup();
        let sliced = if history.len() > 5 {
            &history[history.len() - 5..]
        } else {
            &history[..]
        };
        let s = serde_json::to_string(sliced).unwrap();
        setItem("history", &s);
    }
    let error = create_signal(ctx, false);
    let data = String::from_utf8(base64::decode(&token).unwrap_or_else(|_| {
        error.set(true);
        Default::default()
    }))
    .unwrap_or_else(|_| {
        error.set(true);
        Default::default()
    });
    let deck: Deck = serde_json::from_str(&data).unwrap_or_else(|_| {
        error.set(true);
        Default::default()
    });
    let deck_len = deck.len();
    let deck = create_signal(ctx, deck);
    log(&format!("{:#?}", *deck));
    let current = create_signal(ctx, 0usize);

    deck.modify().shuffle(&mut rand::thread_rng());

    let decrement = move |_| {
        let rn = *current.get();
        if rn == 0 {
            current.set(deck_len - 1);
        } else {
            *current.modify() -= 1;
        }
    };
    let increment = move |_| {
        let rn = *current.get();
        if rn == (deck_len - 1) {
            current.set(0);
        } else {
            *current.modify() += 1;
        }
    };

    let reshuffle = |_| deck.modify().shuffle(&mut rand::thread_rng());

    let on_click = |event: Event| {
        let elem = event
            .current_target()
            .unwrap()
            .dyn_ref::<Element>()
            .unwrap()
            .clone();

        elem.class_list().toggle("flip").unwrap();
    };

    let go_home = |_| set_location("?home");

    let go_modify = move |_| {
        let location = format!("?edit={}", token);
        set_location(&location);
    };

    view! {ctx,
        div(class="text-align-center") {
            button(on:click=go_home) { "Home" }
            button(on:click=go_modify) { "Edit Deck" }
            br
            button(on:click=decrement) {"Prev"}
            button(on:click=reshuffle) {"Reshuffle"}
            button(on:click=increment) { "Next" }
        }
        (if *error.get() {
            view! {ctx,
                p(style="color: red;") {"Something went wrong. Make sure your deck code is valid."}
            }
        } else {view!{ctx,}})
        ({
            let current_card = deck.get().get(*current.get()).cloned().unwrap_or_default();
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
    let front = create_signal(ctx, String::new());
    let back = create_signal(ctx, String::new());

    let error_empty = create_signal(ctx, false);
    let error_parse = create_signal(ctx, false);

    let cards = create_signal(ctx, Vec::new());

    if let Some(token) = get_edit_token() {
        let stripped = token
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let decoded = base64::decode(stripped.as_bytes()).unwrap_or_default();
        let parsed = String::from_utf8(decoded).unwrap_or_default();
        let items: Vec<Card> = serde_json::from_str(&parsed).unwrap_or_default();
        *cards.modify() = items;
    }

    let do_import = |_| {
        let p = prompt("Deck code:");
        if let Some(inp) = p {
            let stripped = inp
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();
            let f = format!("?deck={}", stripped);
            set_location(&f);
        }
    };

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

    let do_use_current = |_| {
        let d = Deck((*cards.get()).clone());

        if d.is_empty() {
            return;
        }

        let r = serde_json::to_string(&d);

        if let Ok(s) = r {
            error_parse.set(false);
            let e = base64::encode(s.as_bytes());
            let f = format!("?deck={}", e);
            set_location(&f);
        } else {
            error_parse.set(true);
        }
    };

    let do_delete_last = |_| {
        cards.modify().pop();
    };

    let open_history = |_| set_location("?history");

    view! {ctx,
        div(class="text-align-center") {
            button(on:click=do_import) {"Import"}
            button(on:click=do_use_current) {"Use Current Deck"}
            button(on:click=open_history) {"Past decks"}
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
            button(on:click=do_delete_last) {"Delete Last"}
            Indexed {
                iterable: cards,
                view: |ctx, card| view! {ctx,
                    div(class="card", style="background-color: white; padding: 5px; transform: none") {
                        "Front: "
                        (card.front)
                        br
                        "Back: "
                        (card.back)
                    }
                }
            }
        }
    }
}

#[component]
fn HistoryComponent<G: Html>(ctx: Scope) -> View<G> {
    let history = create_signal(ctx, {
        let prev = getItem("history").unwrap_or_else(|| String::from("[]"));
        let parsed: Vec<String> = serde_json::from_str(&prev).unwrap_or_default();
        parsed
    });

    let go_home = |_| set_location("?home");

    view! {ctx,
        div(class="text-align-center") {
            button(on:click=go_home) {"Home"}
        }
        Indexed {
            iterable: history,
            view: |ctx, item| view!{ctx,
                div(class="card", style="background-color: white; padding: 5px; transform: none; word-break: break-all ") {
                    (item)
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
                } else if check_history() {
                    view! {ctx, HistoryComponent {}}
                } else {
                    view! {ctx, CreatorComponent {}}
                })
            }
        }
    });
}

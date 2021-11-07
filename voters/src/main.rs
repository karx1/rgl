use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;

fn main() {
    let items = Signal::new(Vec::new());
    let length = Signal::new(0u8);
    let error = Signal::new(String::new());

    spawn_local(cloned!((length, error) => async move {
        let result = async move {
            let resp = surf::get(format!("{}/polls/items.json", env!("FIREBASE_URL"))).recv_string().await?;
            let parsed = json::parse(&resp)?;

            let num = match parsed.as_u8() {
                Some(val) => val,
                None => 0
            };

            length.set(num);

            Ok::<(), Box<dyn ::std::error::Error>>(())
        }.await;

        if let Err(_) = result {
            error.set(String::from("Failed to get data from API"));
        }
    }));

    create_effect(cloned!((items, length, error) => move || {
        let num = *length.get();

        if num > 0 {
            spawn_local(cloned!((items, error) => async move {
                let result = async move {
                    let mut builder = Vec::new();
                    for i in 0..num {
                        let resp = surf::get(format!("{}/polls/{}/description.json", env!("FIREBASE_URL"), i)).recv_string().await?;
                        let parsed = json::parse(&resp)?;

                        let string = String::from(match parsed.as_str() {
                            Some(val) => val,
                            None => ""
                        });

                        builder.push(string);
                    }

                    items.set(builder);

                    Ok::<(), Box<dyn ::std::error::Error>>(())
                }.await;

                if let Err(_) = result {
                    error.set(String::from("Failed to get data from API"));
                }
            }));
        }
    }));

    sycamore::render(|| {
        template! {
            div(class="wrapper") {
                (length.get())
                ul {
                    // (templates)
                    Keyed(KeyedProps {
                        iterable: items.handle(),
                        template: |x| template! {
                            li {(x)}
                        },
                        key: |x| (*x).clone(),
                    })
                }
            }

            (if *error.get() != "" {
                template! { div(class="footer") { "Failed to get data from API" } }
            } else {
                template!{}
            })
        }
    });
}

use sycamore::prelude::*;

fn main() {
    let inp = Signal::new(String::new());
    let err = Signal::new(false);
    let decimal = create_memo(cloned!((inp, err) => move || {
        if *inp.get() == "" {
            return 0;
        }
        if let Ok(parsed) = u128::from_str_radix(&*inp.get(), 2) {
            err.set(false);
            return parsed;
        } else {
            err.set(true);
            return 0;
        }
    }));

    sycamore::render(|| {
        template! {
            h1(style="text-align: center") { "Bin2Dec" }
            div(class="wrapper", style="text-align: center") {
                input(placeholder="Binary", bind:value=inp, style="text-align: center")
                (if *err.get() {
                    template! {
                        span(style="color: red") { "Unable to convert to decimal" }
                    }
                } else {
                    template! {}
                })
                div(class="card") { (decimal.get()) }
            }
            div(class="footer") {
                "Powered by Rust 1.56 and WASM"
            }
        }
    })
}

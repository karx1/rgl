use sycamore::prelude::*;

fn main() {
    let inp = Signal::new(String::new());
    let err = Signal::new(false);
    let double = create_memo(cloned!((inp, err) => move || {
        if *inp.get() == "" {
            return 0;
        }
        if let Ok(parsed) = (*inp.get()).parse::<u64>() {
            err.set(false);
            return parsed * 2;
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
                        span(style="color: red") { "unable to convert" }
                    }
                } else {
                    template! {}
                })
                div(class="card") { (double.get()) }
            }
        }
    })
}

# RemembeRS

[Live App](https://etc.karx.xyz/rgl/remembers/)

## Running Locally
1. Install dependencies

    First, install `rust` from [the Rust website](https://www.rust-lang.org/). Then, install `trunk`:
    ```bash
    cargo install --locked trunk
    ```
    This project requires `rustc` version `1.59.0 stable` because it uses the 2021 edition of Rust and sycamore 0.8!
2. Build project
    ```bash
    cd /path/to/remembers
    trunk serve
    ```
3. Open in browser

    [Check the supported browser list](https://rustwasm.github.io/docs/wasm-bindgen/reference/browser-support.html) and open https://localhost:8080 in one of the supported browsers.
    

<small>Created using [Sycamore](https://crates.io/crates/sycamore) and Rust with WebAssembly</small>

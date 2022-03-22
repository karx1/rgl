# CryptRS

[Live App](https://etc.karx.xyz/rgl/cryptrs)

## Implementation details
This app uses Rust's type system to easily convert between characters and numeric types. After that, it's just a simple Vignere table lookup and then conversion back to a numeric type.

## Running Locally
1. Install dependencies
    First, install `rust` from [the Rust website](https://www.rust-lang.org/). Then, install `trunk`:
    ```bash
    cargo install --locked trunk
    ```
    This project requires `rustc` version `1.57.0 stable` because it uses the 2021 edition of Rust!
2. Build project
    ```bash
    cd /path/to/cryptrs
    trunk serve
    ```
3. Open in browser

    [Check the supported browser list](https://rustwasm.github.io/docs/wasm-bindgen/reference/browser-support.html) and open https://localhost:8080 in one of the supported browsers.


<small>Created using [Sycamore](https://crates.io/crates/sycamore) and Rust with WebAssembly</small>

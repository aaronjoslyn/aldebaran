# rust-wasm-opengl

Auto-reloading Rust in the browser, using tokio, wasm and more.

## Run

First, add wasm32 as a target:

```
$ rustup target add wasm32-unknown-unknown
```

Then, start the server:

```
$ cargo run --bin server
```

Then open http://localhost:3000 in your web browser of choice. Any changes made to the `app` project should be automatically reloaded in the browser.

# spooks-wasm-src
Spook with WASM

  * `cargo web build` - will build 
  * `cargo web start` - will build  project, start an embedded webserver and will continuously
    rebuild it if necessary; supports automatic reloading with `--auto-reload`.
  * `cargo web deploy` - will build your project and emit all of the necessary files so that
    you can easily serve them statically.

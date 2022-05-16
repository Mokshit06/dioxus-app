# Dioxus SSR + Hydration

Trying out making dioxus ssr and hyration work in a single app

- The `app` crate is the shared crate that exposes an `App` fn that contains the rendering + app logic
- The `web` crate bundles the assets + styles and wasm required for client interaction.
- The `ssr` crate contains the server logic and the handler required for rendering the app.

## Usage

- in one terminal session build the wasm output in watch mode using trunk:
  ```bash
  cd crates/web && trunk watch
  ```
- in another terminal session start the server
  ```bash
  cd crates/ssr && cargo run
  ```
- open the app in your browser on the port showed by rocket (usually 8080)

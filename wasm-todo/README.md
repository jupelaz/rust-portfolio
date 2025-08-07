# wasm-todo

ToDo application written in Rust using Yew + WebAssembly.

## Requirements

- [Rust](https://rustup.rs)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- `wasm32` target:  
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

## Run

```bash
trunk serve
```

Open http://localhost:8080

## Build for production

```bash
trunk build --release
```

### 📦 Deployment

- **Firebase Hosting**: run `trunk build --release` and then upload the `dist/` folder as static hosting.
- Also works on Netlify, Vercel or GitHub Pages.
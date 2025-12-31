# Quickstart: Slint 1.14 Web Capabilities Showcase

**Feature**: 004-slint-web-showcase
**Date**: 2026-01-01

## Prerequisites

- Rust latest stable: `rustup update stable`
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- wasm-bindgen CLI: `cargo install wasm-bindgen-cli`
- Node.js 18+ (for frontend tooling, optional)

## Development Setup

### 1. Clone and Checkout

```bash
git checkout 004-slint-web-showcase
```

### 2. Build WASM

```bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir frontend/pkg --target web target/wasm32-unknown-unknown/release/slint_showcase.wasm
```

### 3. Serve Development

```bash
# Using any static file server
cd frontend
npx serve .
# OR
python3 -m http.server 8080
```

### 4. Access Showcase

Open `http://localhost:8080` in browser.

## Project Structure

```
src/
├── lib/
│   ├── components/
│   │   ├── demo_*.rs      # Individual capability demos
│   │   └── code_editor.rs # Editable code example component
│   ├── data/
│   │   ├── capabilities.rs # Demo metadata
│   │   └── examples.rs     # Code example data
│   └── lib.rs             # Library root
├── bin/
│   └── showcase.rs        # Binary entry point
└── wasm/
    └── glue.js            # WASM initialization

frontend/
├── index.html             # Entry HTML
├── pkg/                   # Generated wasm-bindgen output
└── styles/
    └── main.css           # Presentation styles
```

## Adding a New Demo

1. Create Slint file in `src/assets/demos/`
2. Add component to `src/lib/components/`
3. Register in `src/lib/data/capabilities.rs`
4. Build and verify

## Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# WASM tests
cargo test --target wasm32-unknown-unknown
```

## Deployment

```bash
# Production build
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir dist/pkg --target web target/wasm32-unknown-unknown/release/slint_showcase.wasm

# Deploy dist/ to static host (GitHub Pages, Vercel, Netlify)
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| WASM won't load | Verify `Application::new()` is called after WASM initialization |
| Slow compilation | Enable LTO in Cargo.toml: `lto = true` |
| Missing styles | Check `index.html` loads main.css |

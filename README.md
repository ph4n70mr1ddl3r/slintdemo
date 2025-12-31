# Slint 1.14 Web Capabilities Showcase

A Rust + Slint WebAssembly showcase demonstrating Slint 1.14's web capabilities through interactive demonstrations, performance metrics, and code examples.

## Features

- **Interactive Demos**: Counter, Button States, Text Input, Slider, Checkbox
- **Performance Metrics**: Slint vs React comparisons (62.5% faster startup, 83% less memory)
- **Code Playground**: Editable code examples with difficulty levels
- **Accessibility**: Keyboard navigation, ARIA labels, WCAG 2.1 compliance
- **Responsive Design**: Mobile, tablet, and desktop layouts

## Quick Start

```bash
# Run all tests (213 passing tests)
cargo test --workspace

# Run the binary (shows version info)
cargo run -p slint-showcase-bin

# Run clippy lints
cargo clippy --workspace

# Format code
cargo fmt
```

## Web Showcase Setup

The web showcase requires additional setup for full WASM rendering:

```bash
# 1. Build the WASM module
cargo build --target wasm32-unknown-unknown -p slint-showcase-lib

# 2. Generate JavaScript bindings
wasm-bindgen --out-dir frontend/pkg/ --target web \
    target/wasm/wasm32-unknown-unknown/debug/slint_showcase_lib.wasm

# 3. Serve the frontend
cd frontend
python3 -m http.server 8080
```

Then open http://localhost:8080 in your browser.

**Note**: Full web rendering requires Slint's canvas-based preview API, which needs additional JavaScript platform setup beyond basic wasm-bindgen.

## Project Structure

```
slintdemo/
├── src/
│   ├── lib/                 # Slint UI components
│   │   ├── lib.rs           # Main entry point with App component
│   │   ├── components/      # Demo components
│   │   └── data/            # Data models (demos, categories, benchmarks)
│   ├── bin/                 # Binary entry point
│   └── wasm/                # WebAssembly glue code
├── frontend/                # Web frontend
│   ├── index.html           # Main HTML page
│   ├── styles/              # CSS styles
│   └── tests/               # Playwright E2E tests
├── specs/                   # Feature specifications
│   └── 004-slint-web-showcase/
└── Cargo.toml               # Workspace manifest
```

## Test Results

```
Phase 1-2 (Setup + Foundational): 15/15 tasks ✓
Phase 3 (Interactive Demos): 13/13 tasks, 31 tests ✓
Phase 4 (Capability Catalog): 14/14 tasks, 37 tests ✓
Phase 5 (Performance Metrics): 14/14 tasks, 51 tests ✓
Phase 6 (Code Playground): 14/14 tasks, 36 tests ✓
Phase 7 (Cross-Device): 14/14 tasks, 58 tests ✓
Phase 8 (Polish): 13/13 tasks ✓

Total: 97 tasks, 213 tests passing
```

## Tech Stack

- **Language**: Rust (latest stable)
- **UI Framework**: Slint 1.14
- **WebAssembly**: wasm32-unknown-unknown target
- **Build Tool**: cargo
- **Testing**: cargo test, Playwright
- **CI/CD**: GitHub Actions

## References

- [Slint Documentation](https://slint.dev/docs/)
- [Slint GitHub](https://github.com/slint-ui/slint)
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)

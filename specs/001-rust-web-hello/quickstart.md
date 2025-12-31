# Quickstart: Rust Web Hello World

## Prerequisites

- Rust latest stable (`rustup update stable`)
- Cargo (comes with Rust)

## Setup

```bash
# Clone the repository
git clone <repo-url>
cd slintdemo

# Checkout the feature branch
git checkout 001-rust-web-hello
```

## Run the Application

```bash
# Build and run
cargo run

# The server will start on http://localhost:8080
```

## Verify

```bash
# Test the endpoint
curl http://localhost:8080

# Expected output: <html><body><h1>Hello World</h1></body></html>
```

## Run Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

## Project Structure

```
src/
├── main.rs         # Application entry point
└── lib.rs          # Handlers and utilities

tests/
├── unit/
│   └── test_main.rs
└── integration/
    └── test_hello.rs
```

## Next Steps

See `tasks.md` for implementation tasks (run `/speckit.tasks` to generate).

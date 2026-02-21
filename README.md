# Hello World

A minimal Rust web application that serves a "Hello World" page on port 8080.

## Prerequisites

- Rust latest stable (`rustup update stable`)
- Cargo (comes with Rust)

## Setup

```bash
git clone <repo-url>
cd slintdemo
```

## Run the Application

```bash
cargo run
```

The server will start on http://localhost:8080

## Verify

```bash
curl http://localhost:8080
```

Expected output: A full HTML5 document with `<!DOCTYPE html>` and "Hello World" heading

## Run Tests

```bash
cargo test
```

## Project Structure

```
src/
├── main.rs         # Application entry point
└── lib.rs          # Handlers and utilities

tests/
└── test_hello.rs   # Integration tests
```

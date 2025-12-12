# rust-wasm-mental-poker

Fresh Rust implementation of 2‑player mental poker over an EC group (Ristretto/Curve25519),
intended for compilation to WebAssembly.

## Status

This crate wraps the `ziffle` mental‑poker library to provide:

- EC‑ElGamal deck encryption on secp256k1.
- Bayer‑Groth 2012 sigma‑protocol verifiable shuffles (Fiat–Shamir NIZK).
- Sigma proofs for reveal tokens (partial decrypt shares).
- Ordered 2‑player NLHE flow: key setup → initial shuffle (P0) → shuffle (P1) →
  preflop deal → flop/turn/river → openings.

Note: `ziffle` is experimental and explicitly warns it has not been independently
audited. Use for research/demos unless you complete your own review.

## Building

Requires Rust stable. To run tests:

```bash
cargo test
```

To build wasm (browser):

```bash
wasm-pack build --target web
```

You may need to enable `wasm` feature depending on toolchain:

```bash
cargo build --release --target wasm32-unknown-unknown --features wasm
```

## Console simulation via wasm

After building with `wasm-pack`:

```bash
wasm-pack build --target nodejs
node examples/node-sim.mjs
```

This runs a full ordered hand and prints preflop/flop/turn/river to the console.

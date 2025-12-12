import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { simulate_hand_wasm } = require("../pkg/rust_wasm_mental_poker.js");

console.log(simulate_hand_wasm());

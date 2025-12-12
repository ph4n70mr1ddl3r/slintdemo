use crate::protocol::{simulate_hand, GameState};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmGame {
    inner: GameState,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGame {
        let inner = GameState::new(b"browser-session");
        WasmGame { inner }
    }

    pub fn shuffle_p0(&mut self) {
        // Initial shuffle is already done by P0 in GameState::new.
        // This method is kept for symmetry and can be used to reshuffle.
        self.inner.shuffle_by(0);
    }

    pub fn shuffle_p1(&mut self) {
        self.inner.shuffle_by(1);
    }

    pub fn deal(&mut self) {
        self.inner.deal_preflop();
        self.inner.deal_community();
    }

    pub fn hole_cards_p0(&self) -> Vec<u8> {
        self.inner.open_hole_for(0).to_vec()
    }

    pub fn hole_cards_p1(&self) -> Vec<u8> {
        self.inner.open_hole_for(1).to_vec()
    }

    pub fn community_cards(&self) -> Vec<u8> {
        self.inner.open_community().to_vec()
    }
}

#[wasm_bindgen]
pub fn simulate_hand_wasm() -> String {
    simulate_hand(b"browser-session")
}

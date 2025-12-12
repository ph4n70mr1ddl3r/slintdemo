use crate::types::*;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
use js_sys::Date;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use getrandom::getrandom;
use ziffle::{AggregatePublicKey, AggregateRevealToken};

fn rng() -> StdRng {
    let mut seed = [0u8; 32];
    getrandom(&mut seed).expect("getrandom");
    StdRng::from_seed(seed)
}

pub struct Player {
    pub id: u8,
    pub keys: PlayerKeys,
}

pub struct GameState {
    shuffle: ZShuffle,
    ctx: Vec<u8>,
    pub players: [Player; 2],
    pub apk: ZApk,
    pub deck: ZVerifiedDeck,
    pub hole_cards: [Vec<ZMaskedCard>; 2],
    pub community: Vec<ZMaskedCard>,
}

impl GameState {
    pub fn new(ctx: impl AsRef<[u8]>) -> Self {
        let shuffle = ZShuffle::default();
        let ctx_vec = ctx.as_ref().to_vec();
        let mut r = rng();

        // Keygen for both players, with ownership proofs verified by the other.
        let (sk0, pk0, proof0) = shuffle.keygen(&mut r, &ctx_vec);
        let (sk1, pk1, proof1) = shuffle.keygen(&mut r, &ctx_vec);
        let vpk0 = proof0.verify(pk0, &ctx_vec).expect("vpk0");
        let vpk1 = proof1.verify(pk1, &ctx_vec).expect("vpk1");

        let apk = AggregatePublicKey::new(&[vpk0, vpk1]);
        let p0 = Player {
            id: 0,
            keys: PlayerKeys { sk: sk0, pk: pk0, ownership_proof: proof0, verified_pk: vpk0 },
        };
        let p1 = Player {
            id: 1,
            keys: PlayerKeys { sk: sk1, pk: pk1, ownership_proof: proof1, verified_pk: vpk1 },
        };

        // Initial shuffle by player 0, verified by player 1.
        let mut r2 = rng();
        let (deck0, proof_s0) = shuffle.shuffle_initial_deck(&mut r2, apk, &ctx_vec);
        let vdeck0 = shuffle
            .verify_initial_shuffle(apk, deck0, proof_s0, &ctx_vec)
            .expect("initial shuffle verify");

        Self {
            shuffle,
            ctx: ctx_vec,
            players: [p0, p1],
            apk,
            deck: vdeck0,
            hole_cards: [Vec::new(), Vec::new()],
            community: Vec::new(),
        }
    }

    pub fn shuffle_by(&mut self, _shuffler: u8) -> ShuffledDeck {
        let mut r = rng();
        let prev = self.deck;
        let (next, proof) = self.shuffle.shuffle_deck(&mut r, self.apk, &prev, &self.ctx);
        let vnext = self
            .shuffle
            .verify_shuffle(self.apk, &prev, next, proof, &self.ctx)
            .expect("shuffle verify");
        self.deck = vnext;
        ShuffledDeck { deck: next, proof }
    }

    pub fn deal_preflop(&mut self) {
        self.hole_cards = [Vec::new(), Vec::new()];
        for round in 0..2 {
            for pid in 0..2 {
                let idx = round * 2 + pid;
                let card = self.deck.get(idx).expect("deck card");
                self.hole_cards[pid].push(card);
            }
        }
    }

    pub fn deal_community(&mut self) {
        self.community.clear();
        for idx in 4..9 {
            let card = self.deck.get(idx).expect("community card");
            self.community.push(card);
        }
    }

    pub fn open_card(&self, card: ZMaskedCard) -> CardId {
        let mut r = rng();
        let (t0, p0) = card.reveal_token(&mut r, &self.players[0].keys.sk, self.players[0].keys.pk, &self.ctx);
        let (t1, p1) = card.reveal_token(&mut r, &self.players[1].keys.sk, self.players[1].keys.pk, &self.ctx);
        let vt0 = p0
            .verify(self.players[0].keys.verified_pk, t0, card, &self.ctx)
            .expect("verify token 0");
        let vt1 = p1
            .verify(self.players[1].keys.verified_pk, t1, card, &self.ctx)
            .expect("verify token 1");
        let at = AggregateRevealToken::new(&[vt0, vt1]);
        let idx = self.shuffle.reveal_card(at, card).expect("reveal card");
        u8::try_from(idx).expect("card id fits u8")
    }

    pub fn open_hole_for(&self, pid: u8) -> [CardId; 2] {
        let cards = &self.hole_cards[pid as usize];
        [self.open_card(cards[0]), self.open_card(cards[1])]
    }

    pub fn open_community(&self) -> [CardId; 5] {
        [
            self.open_card(self.community[0]),
            self.open_card(self.community[1]),
            self.open_card(self.community[2]),
            self.open_card(self.community[3]),
            self.open_card(self.community[4]),
        ]
    }
}

pub fn simulate_hand(ctx: impl AsRef<[u8]>) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    fn fmt_ms(ms: f64) -> String {
        format!("{ms:.2}ms")
    }
    #[cfg(target_arch = "wasm32")]
    fn fmt_ms(ms: f64) -> String {
        format!("{ms:.2}ms")
    }

    #[cfg(target_arch = "wasm32")]
    fn now_ms() -> f64 {
        Date::now()
    }

    #[cfg(target_arch = "wasm32")]
    let t_total0 = now_ms();
    #[cfg(not(target_arch = "wasm32"))]
    let t_total0 = Instant::now();

    #[cfg(target_arch = "wasm32")]
    let t_new0 = now_ms();
    #[cfg(not(target_arch = "wasm32"))]
    let t_new0 = Instant::now();

    let mut gs = GameState::new(ctx);

    #[cfg(target_arch = "wasm32")]
    let dt_new = now_ms() - t_new0;
    #[cfg(not(target_arch = "wasm32"))]
    let dt_new = t_new0.elapsed().as_secs_f64() * 1000.0;

    #[cfg(target_arch = "wasm32")]
    let t_shuffle10 = now_ms();
    #[cfg(not(target_arch = "wasm32"))]
    let t_shuffle10 = Instant::now();

    gs.shuffle_by(1);

    #[cfg(target_arch = "wasm32")]
    let dt_shuffle1 = now_ms() - t_shuffle10;
    #[cfg(not(target_arch = "wasm32"))]
    let dt_shuffle1 = t_shuffle10.elapsed().as_secs_f64() * 1000.0;

    #[cfg(target_arch = "wasm32")]
    let t_deal0 = now_ms();
    #[cfg(not(target_arch = "wasm32"))]
    let t_deal0 = Instant::now();

    gs.deal_preflop();
    gs.deal_community();

    #[cfg(target_arch = "wasm32")]
    let dt_deal = now_ms() - t_deal0;
    #[cfg(not(target_arch = "wasm32"))]
    let dt_deal = t_deal0.elapsed().as_secs_f64() * 1000.0;

    #[cfg(target_arch = "wasm32")]
    let t_open_hole00 = now_ms();
    #[cfg(not(target_arch = "wasm32"))]
    let t_open_hole00 = Instant::now();

    let h0 = gs.open_hole_for(0);

    #[cfg(target_arch = "wasm32")]
    let dt_open_hole0 = now_ms() - t_open_hole00;
    #[cfg(not(target_arch = "wasm32"))]
    let dt_open_hole0 = t_open_hole00.elapsed().as_secs_f64() * 1000.0;

    #[cfg(target_arch = "wasm32")]
    let t_open_hole10 = now_ms();
    #[cfg(not(target_arch = "wasm32"))]
    let t_open_hole10 = Instant::now();

    let h1 = gs.open_hole_for(1);

    #[cfg(target_arch = "wasm32")]
    let dt_open_hole1 = now_ms() - t_open_hole10;
    #[cfg(not(target_arch = "wasm32"))]
    let dt_open_hole1 = t_open_hole10.elapsed().as_secs_f64() * 1000.0;

    #[cfg(target_arch = "wasm32")]
    let t_open_comm0 = now_ms();
    #[cfg(not(target_arch = "wasm32"))]
    let t_open_comm0 = Instant::now();

    let comm = gs.open_community();

    #[cfg(target_arch = "wasm32")]
    let dt_open_comm = now_ms() - t_open_comm0;
    #[cfg(not(target_arch = "wasm32"))]
    let dt_open_comm = t_open_comm0.elapsed().as_secs_f64() * 1000.0;

    let flop = &comm[0..3];
    let turn = comm[3];
    let river = comm[4];

    #[cfg(target_arch = "wasm32")]
    let dt_total = now_ms() - t_total0;
    #[cfg(not(target_arch = "wasm32"))]
    let dt_total = t_total0.elapsed().as_secs_f64() * 1000.0;

    format!(
        "=== Timings ===\nsetup+initial shuffle (P0): {}\nshuffle (P1): {}\ndeal: {}\nopen hole P0: {}\nopen hole P1: {}\nopen community: {}\ntotal: {}\n\n=== Preflop ===\nP0: {:?}\nP1: {:?}\n=== Flop ===\n{:?}\n=== Turn ===\n{:?}\n=== River ===\n{:?}\n",
        fmt_ms(dt_new),
        fmt_ms(dt_shuffle1),
        fmt_ms(dt_deal),
        fmt_ms(dt_open_hole0),
        fmt_ms(dt_open_hole1),
        fmt_ms(dt_open_comm),
        fmt_ms(dt_total),
        h0,
        h1,
        flop,
        turn,
        river
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};

    #[test]
    fn full_hand_with_real_proofs() {
        let mut gs = GameState::new(b"test-session");
        gs.shuffle_by(1); // second player shuffle after initial
        gs.deal_preflop();
        gs.deal_community();
        let h0 = gs.open_hole_for(0);
        let h1 = gs.open_hole_for(1);
        let comm = gs.open_community();
        assert_ne!(h0, h1);
        assert_eq!(comm.len(), 5);
    }

    #[test]
    fn tampered_shuffle_deck_fails_verification() {
        let gs = GameState::new(b"tamper-deck");
        let mut r = rng();
        let prev = gs.deck;
        let (next, proof) = gs
            .shuffle
            .shuffle_deck(&mut r, gs.apk, &prev, &gs.ctx);

        let mut bytes = Vec::new();
        next.serialize_with_mode(&mut bytes, Compress::Yes)
            .expect("serialize deck");
        bytes[10] ^= 0x55;
        let tampered_res: Result<ziffle::MaskedDeck<52>, _> =
            CanonicalDeserialize::deserialize_with_mode(
                &mut bytes.as_slice(),
                Compress::Yes,
                Validate::Yes,
            );
        match tampered_res {
            Err(_) => return, // tamper detected by canonical decoding
            Ok(tampered) => {
                let res = gs
                    .shuffle
                    .verify_shuffle(gs.apk, &prev, tampered, proof, &gs.ctx);
                assert!(res.is_none());
            }
        }
    }

    #[test]
    fn tampered_shuffle_proof_fails_verification() {
        let gs = GameState::new(b"tamper-proof");
        let mut r = rng();
        let prev = gs.deck;
        let (next, proof) = gs
            .shuffle
            .shuffle_deck(&mut r, gs.apk, &prev, &gs.ctx);

        let mut bytes = Vec::new();
        proof
            .serialize_with_mode(&mut bytes, Compress::Yes)
            .expect("serialize proof");
        bytes[5] ^= 0xAA;
        let tampered_proof_res: Result<ziffle::ShuffleProof<52>, _> =
            CanonicalDeserialize::deserialize_with_mode(
                &mut bytes.as_slice(),
                Compress::Yes,
                Validate::Yes,
            );
        match tampered_proof_res {
            Err(_) => return,
            Ok(tampered_proof) => {
                let res = gs
                    .shuffle
                    .verify_shuffle(gs.apk, &prev, next, tampered_proof, &gs.ctx);
                assert!(res.is_none());
            }
        }
    }

    #[test]
    fn tampered_reveal_token_fails_verification() {
        let mut gs = GameState::new(b"tamper-reveal");
        gs.shuffle_by(1);
        gs.deal_preflop();
        let card = gs.hole_cards[0][0];
        let mut r = rng();
        let (token0, proof0) = card.reveal_token(
            &mut r,
            &gs.players[0].keys.sk,
            gs.players[0].keys.pk,
            &gs.ctx,
        );

        let mut bytes = Vec::new();
        token0
            .serialize_with_mode(&mut bytes, Compress::Yes)
            .expect("serialize token");
        bytes[0] ^= 0xFF;
        let tampered_token_res: Result<ziffle::RevealToken, _> =
            CanonicalDeserialize::deserialize_with_mode(
                &mut bytes.as_slice(),
                Compress::Yes,
                Validate::Yes,
            );
        match tampered_token_res {
            Err(_) => return,
            Ok(tampered_token) => {
                let res = proof0.verify(
                    gs.players[0].keys.verified_pk,
                    tampered_token,
                    card,
                    &gs.ctx,
                );
                assert!(res.is_none());
            }
        }
    }
}

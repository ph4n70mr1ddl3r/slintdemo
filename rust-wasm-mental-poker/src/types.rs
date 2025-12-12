use ziffle::{
    AggregatePublicKey, MaskedCard, MaskedDeck, OwnershipProof, PublicKey, RevealToken,
    RevealTokenProof, SecretKey, Shuffle, ShuffleProof, Verified,
};

pub type CardId = u8;

pub struct PlayerKeys {
    pub sk: SecretKey,
    pub pk: PublicKey,
    pub ownership_proof: OwnershipProof,
    pub verified_pk: Verified<PublicKey>,
}

pub struct ShuffledDeck {
    pub deck: MaskedDeck<52>,
    pub proof: ShuffleProof<52>,
}

pub struct RevealShare {
    pub token: RevealToken,
    pub proof: RevealTokenProof,
}

pub enum Message {
    PublishKey { from: u8, pk: PublicKey, proof: OwnershipProof },
    Shuffle { from: u8, deck: MaskedDeck<52>, proof: ShuffleProof<52> },
    RevealToken { from: u8, index: u8, token: RevealToken, proof: RevealTokenProof },
}

pub(crate) type ZShuffle = Shuffle<52>;
pub(crate) type ZVerifiedDeck = Verified<MaskedDeck<52>>;
pub(crate) type ZMaskedCard = MaskedCard;
pub(crate) type ZApk = AggregatePublicKey;

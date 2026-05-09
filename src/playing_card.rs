use std::fmt::Display;
use strum_macros::Display;
use crate::shithead_error::ShitheadError;

#[derive(Copy, Clone, Display)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

pub struct PlayingCard {
    suit: Suit,
    rank: u8,
}

impl PlayingCard {
    pub const MIN_RANK: u8 = 1;
    pub const MAX_RANK: u8 = 13;

    pub fn new(suit: Suit, rank: u8) -> Result<PlayingCard, ShitheadError> {
        if (rank < PlayingCard::MIN_RANK) || (rank > PlayingCard::MAX_RANK) {
            return Err(ShitheadError::InvalidRank { rank });
        }

        Ok(Self { suit, rank })
    }
}

impl Display for PlayingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("(Suit={}, Rank={})", self.suit, self.rank))
    }
}

use std::process::{ExitCode, Termination};
use strum_macros::{Display};

enum ShitheadError {
    InvalidRank { rank: u8 },
}

impl Termination for ShitheadError {
    fn report(self) -> ExitCode {
        match self {
            Self::InvalidRank {rank : _} => ExitCode::from(1),
        }
    }
}

#[derive(Copy, Clone, Display)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

struct PlayingCard {
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

    pub fn as_str(&self) -> String {
        format!("(Suit={}, Rank={})", self.suit, self.rank)
    }
}

fn play_shithead() -> Result<(), ShitheadError> {
    for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
        for rank in PlayingCard::MIN_RANK..=PlayingCard::MAX_RANK {
            let card = PlayingCard::new(suit, rank)?;
            println!("{}", card.as_str())
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match play_shithead() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => e.report(),
    }
}

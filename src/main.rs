pub mod playing_card;
pub mod shithead_error;

use crate::shithead_error::ShitheadError;
use crate::playing_card::{Suit, PlayingCard};

use std::process::{ExitCode, Termination};



fn play_shithead() -> Result<(), ShitheadError> {
    for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
        for rank in PlayingCard::MIN_RANK..=PlayingCard::MAX_RANK {
            let card = PlayingCard::new(suit, rank)?;
            println!("{}", card.to_string())
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

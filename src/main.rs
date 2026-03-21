enum PlayingCardError {
    InvalidRank {rank: u8},
}

impl PlayingCardError {
    pub fn as_str(&self) -> String {
        match self {
            Self::InvalidRank { rank } => format!("InvalidRank received {}", rank),
        }
    }
}

#[derive(Copy, Clone)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Suit::Spades => "Spades",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs"
        }
    }
}

struct PlayingCard {
    suit: Suit,
    rank: u8
}

impl PlayingCard
{
    pub const MIN_RANK: u8 = 1;
    pub const MAX_RANK: u8 = 13;

    pub fn new(suit: Suit, rank: u8) -> Result<PlayingCard, PlayingCardError>
    {
        if (rank < PlayingCard::MIN_RANK) |
            (rank > PlayingCard::MAX_RANK) {
            return Err(PlayingCardError::InvalidRank{rank})
        }

        Ok(Self {
            suit,
            rank
        })
    }

    pub fn as_str(&self) -> String {
        format!("(Suit={}, Rank={})", self.suit.as_str(), self.rank)
    }
}

fn unsafe_main() -> Result<(), PlayingCardError>  {
    for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
        for rank in PlayingCard::MIN_RANK..=PlayingCard::MAX_RANK
        {
            let card = PlayingCard::new(suit, rank)?;
            println!("{}", card.as_str())
        }
    }

    Ok(())
}

fn main() {
    // TODO: Figure out how to handle different error signatures. PlayingCard::new returns 2
    //  parameters but another function may return a different number or different types
    match unsafe_main() {
        Ok(_) => (),
        Err(e) => println!("Caught error {}", e.as_str())
    }
}
use std::process::{ExitCode, Termination};

pub enum ShitheadError {
    InvalidRank { rank: u8 },
}

impl Termination for ShitheadError {
    fn report(self) -> ExitCode {
        match self {
            Self::InvalidRank {rank : invalid_rank} => {
                println!("Invalid rank: {invalid_rank}");
                ExitCode::from(1)
            },
        }
    }
}

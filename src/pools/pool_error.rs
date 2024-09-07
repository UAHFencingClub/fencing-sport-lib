use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub enum PoolSheetError {
    UnsupportedParticipantCount,
    InvalidBout,
    NoBoutFound,
    PoolNotComplete,
    // Need to reevaluate there datatructure since I might want to rewrite how the bout structure is done and worked with.
    UnableToCompleteBout_REEVALUATE,
}

impl Display for PoolSheetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PoolSheetError::*;
        match self {
            UnableToCompleteBout_REEVALUATE => write!(
                f,
                "a winner is not able to be determined for the given bout data"
            ),
            InvalidBout => write!(f, "this bout is invalid for this poolsheet"),
            NoBoutFound => write!(f, "this bout does not exist in this poolsheet"),
            PoolNotComplete => write!(f, "the poolsheet has incomplete bouts"),
            UnsupportedParticipantCount => write!(
                f,
                "a poolsheet cannot be generated with the given amount of fencers"
            ),
        }
    }
}

impl Error for PoolSheetError {}

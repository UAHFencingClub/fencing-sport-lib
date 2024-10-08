use std::{error::Error, fmt::Display};

use crate::bout::VersusError;

#[derive(Debug, Clone)]
pub enum PoolSheetError {
    UnsupportedParticipantCount,
    InvalidBout,
    NoBoutFound,
    PoolNotComplete(Vec<usize>),
}

impl Display for PoolSheetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PoolSheetError::*;
        match self {
            InvalidBout => write!(f, "the requested bout is invalid"),
            NoBoutFound => write!(f, "this bout does not exist in this poolsheet"),
            PoolNotComplete(indexes) => {
                write!(
                    f,
                    "the poolsheet has incomplete bouts, indexes: {indexes:?}"
                )
            }
            UnsupportedParticipantCount => write!(
                f,
                "a poolsheet cannot be generated with the given amount of fencers"
            ),
        }
    }
}

impl Error for PoolSheetError {}

impl From<VersusError> for PoolSheetError {
    fn from(value: VersusError) -> Self {
        match value {
            VersusError::SameFencer => PoolSheetError::InvalidBout,
        }
    }
}

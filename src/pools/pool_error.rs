#[derive(Debug, Clone, Copy)]
pub enum PoolSheetError {
    UnsupportedParticipantCount,
    InvalidBout,
    NoBoutFound,
    PoolNotComplete,
    // Need to reevaluate there datatructure since I might want to rewrite how the bout structure is done and worked with.
    UnableToCompleteBout_REEVALUATE,
}

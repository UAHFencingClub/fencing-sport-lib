#[derive(Debug, Clone, Copy)]
pub enum PoolSheetError {
    UnsupportedParticipantCount,
    InvalidBout,
    NoBoutFound,
    PoolNotComplete,
}

use crate::{cards::Cards, fencer::Fencer};
use std::{borrow::Borrow, marker::PhantomData};

/// A struct to associate a Fencer with a score and cards.
/// # Usage
/// ```
/// use fencing_sport_lib::{fencer::SimpleFencer, bout::FencerScore, cards::Cards};
/// let fencer_a = SimpleFencer::new("Alice");
///
/// // Like many structs in this library, need to explicitly type it
/// let versus: FencerScore<SimpleFencer, SimpleFencer> = FencerScore::new(fencer_a, 1, Cards::default());
///
/// // Because you can also wrap the fencer type with smart pointers.
/// let fencer_a = Box::new(SimpleFencer::new("Alice"));
/// let versus: FencerScore<SimpleFencer, _> = FencerScore::new(fencer_a, 1, Cards::default());
/// ```
#[derive(Debug, Clone)]
pub struct FencerScore<U: Fencer, T: Borrow<U>> {
    pub(crate) fencer: T,
    pub(crate) score: u8,
    pub(crate) cards: Cards,
    _p: PhantomData<U>,
}

impl<U: Fencer, T: Borrow<U>> FencerScore<U, T> {
    /// Create a new instance of the FencerScore struct.
    pub fn new(fencer: T, score: u8, cards: Cards) -> FencerScore<U, T> {
        FencerScore {
            fencer,
            score,
            cards,
            _p: PhantomData,
        }
    }
}

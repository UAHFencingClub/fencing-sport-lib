use crate::{cards::Cards, fencer::Fencer};
use std::{borrow::Borrow, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct FencerScore<U: Fencer, T: Borrow<U>> {
    pub(crate) fencer: T,
    pub(crate) score: u8,
    pub(crate) cards: Cards,
    _p: PhantomData<U>,
}

impl<U: Fencer, T: Borrow<U>> FencerScore<U, T> {
    pub fn new(fencer: T, score: u8, cards: Cards) -> FencerScore<U, T> {
        FencerScore {
            fencer,
            score,
            cards,
            _p: PhantomData,
        }
    }
}

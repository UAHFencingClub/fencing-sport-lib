use std::{
    fmt,
    hash::{self, Hash},
};

use crate::cards::Cards;
use crate::fencer::Fencer;

#[derive(Debug)]
pub struct FencerScore<T: Fencer> {
    pub fencer: T,
    pub score: u8,
    pub cards: Cards,
}

#[derive(Debug)]
pub struct Bout<T: Fencer> {
    fencers: FencerVs<T>,
    scores: Option<(FencerScore<T>, FencerScore<T>)>,
}

impl<T: Fencer> Bout<T> {
    pub fn update_score(&mut self, score_a: FencerScore<T>, score_b: FencerScore<T>) {
        self.scores = Some((score_a, score_b));
    }

    pub fn new(fencers: FencerVs<T>) -> Self {
        Bout {
            fencers,
            scores: None,
        }
    }
}

#[derive(Debug, Hash)]
pub enum FencerVsError {
    SameFencer,
}

// Written with generative ai
impl fmt::Display for FencerVsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FencerVsError::SameFencer => write!(f, "A fencer cannot fence themselves."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FencerVs<T: Fencer>(T, T);

impl<T: Fencer> FencerVs<T> {
    pub fn new(fencer_a: T, fencer_b: T) -> Result<Self, FencerVsError> {
        if fencer_a == fencer_b {
            return Err(FencerVsError::SameFencer);
        }
        Ok(FencerVs(fencer_a, fencer_b))
    }
}

impl<T: Fencer> Hash for FencerVs<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match self {
            FencerVs(a, b) if a > b => {
                a.hash(state);
                b.hash(state);
            }
            FencerVs(a, b) if b > a => {
                b.hash(state);
                a.hash(state);
            }
            _ => {
                panic!("A FencerVs struct should not have its 2 items be the same.")
            }
        }
    }
}

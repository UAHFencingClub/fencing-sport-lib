use std::{
    fmt,
    hash::{self, Hash},
};

use crate::cards::Cards;
use crate::{
    cards,
    fencer::{self, Fencer},
};

#[derive(Debug)]
pub struct FencerScore<'a, T: Fencer> {
    pub fencer: &'a T,
    pub score: u8,
    pub cards: Cards,
}

impl<'a, T: Fencer> FencerScore<'a, T> {
    pub fn new(fencer: &'a T, score: u8) -> Self {
        FencerScore {
            fencer,
            score,
            cards: Cards::default(),
        }
    }
}

#[derive(Debug)]
pub struct Bout<'a, T: Fencer> {
    fencers: FencerVs<'a, T>,
    scores: Option<(u8, u8)>,
}

impl<'a, T: Fencer> Bout<'a, T> {
    pub fn update_score(
        &mut self,
        score_a: &FencerScore<'a, T>,
        score_b: &FencerScore<'a, T>,
    ) -> Result<(), ()> {
        let pos_a = self.fencers.contains_pos(score_a.fencer);
        let pos_b = self.fencers.contains_pos(score_b.fencer);
        if pos_a == pos_b {
            return Err(());
        }

        let score_0;
        let score_1;

        match pos_a {
            TuplePos::First => {
                score_0 = score_a.score;
                score_1 = score_b.score
            }
            TuplePos::Second => {
                score_1 = score_a.score;
                score_0 = score_b.score
            }
            TuplePos::None => return Err(()),
        }

        match pos_b {
            TuplePos::None => return Err(()),
            _ => {}
        }

        self.scores = Some((score_0, score_1));

        Ok(())
    }

    pub fn new(fencers: FencerVs<'a, T>) -> Self {
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

#[derive(PartialEq, Eq)]
enum TuplePos {
    First,
    Second,
    None,
}

// Maybe make this take in boxes to fencers?
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FencerVs<'a, T: Fencer>(pub &'a T, pub &'a T);

impl<'a, T: Fencer> FencerVs<'a, T> {
    pub fn new(fencer_a: &'a T, fencer_b: &'a T) -> Result<Self, FencerVsError> {
        if fencer_a == fencer_b {
            return Err(FencerVsError::SameFencer);
        }
        Ok(FencerVs(fencer_a, fencer_b))
    }

    fn contains_pos(&self, fencer: &T) -> TuplePos {
        if fencer == self.0 {
            TuplePos::First
        } else if fencer == self.1 {
            TuplePos::Second
        } else {
            TuplePos::None
        }
    }
}

impl<'a, T: Fencer> Hash for FencerVs<'a, T> {
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

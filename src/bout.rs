use std::{fmt, hash::{self, Hash}};

use crate::fencer::Fencer;

#[derive(Debug)]
pub struct Bout<T: Fencer>{
    fencers: FencerVs<T>,
    scores: Option<(u8, u8)>,
}

impl<T: Fencer> Bout<T> {
    pub fn update_score(&mut self, score_a: u8, score_b: u8) {
        self.scores = Some((score_a, score_b));
    } 

    pub fn new(fencers: FencerVs<T>) -> Self {
        Bout {
            fencers,
            scores: None,
        }
    }
}

#[derive(Debug)]
#[derive(Hash)]
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

// Maybe make this take in boxes to fencers?
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
pub struct FencerVs<T: Fencer>(T, T);

impl<T: Fencer> FencerVs<T>{
    pub fn new(fencer_a: T, fencer_b: T) -> Result<Self, FencerVsError>{
        if fencer_a == fencer_b {
            return Err(FencerVsError::SameFencer);
        }
        Ok(FencerVs(fencer_a,fencer_b))
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
            _ => {panic!("A FencerVs struct should not have its 2 items be the same.")}
        }
    }
}
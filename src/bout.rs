use std::{fmt, hash::{self, Hash}};

use crate::fencer::Fencer;
use crate::cards::Cards;

#[derive(Debug)]
pub struct FencerScore<'a, T: Fencer> {
    pub fencer: &'a T,
    pub score: u8,
    pub cards: Cards, 
}

#[derive(Debug)]
pub struct Bout<'a, T: Fencer>{
    fencers: FencerVs<'a, T>,
    scores: Option<(FencerScore<'a, T>, FencerScore<'a, T>)>,
}

impl<'a, T: Fencer> Bout<'a, T> {
    pub fn update_score(&mut self, score_a: FencerScore<'a, T>, score_b: FencerScore<'a, T>) {
        self.scores = Some((score_a, score_b));
    } 

    pub fn new(fencers: FencerVs<'a, T>) -> Self {
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
pub struct FencerVs<'a, T: Fencer>(&'a T, &'a T);

impl<'a, T: Fencer> FencerVs<'a, T>{
    pub fn new(fencer_a: &'a T, fencer_b: &'a T) -> Result<Self, FencerVsError>{
        if fencer_a == fencer_b {
            return Err(FencerVsError::SameFencer);
        }
        Ok(FencerVs(fencer_a,fencer_b))
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
            _ => {panic!("A FencerVs struct should not have its 2 items be the same.")}
        }
    }
}
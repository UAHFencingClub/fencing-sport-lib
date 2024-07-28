use crate::fencer::Fencer;
use std::{
    borrow::Borrow,
    fmt::{Display, Formatter, Result as FmtResult},
    hash::{self, Hash},
    marker::PhantomData,
};

#[derive(Debug, Hash)]
pub enum FencerVsError {
    SameFencer,
}

impl Display for FencerVsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FencerVsError::SameFencer => write!(f, "A fencer cannot fence themselves."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum TuplePos {
    First,
    Second,
    None,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FencerVs<U: Fencer, T: Borrow<U>>(T, T, PhantomData<U>);

impl<U: Fencer, T: Borrow<U> + Clone> FencerVs<U, T> {
    pub fn new(fencer_a: T, fencer_b: T) -> Result<Self, FencerVsError> {
        if fencer_a.borrow() == fencer_b.borrow() {
            Err(FencerVsError::SameFencer)
        } else {
            Ok(FencerVs(fencer_a, fencer_b, PhantomData))
        }
    }

    pub fn get_fencer(&self, fencer: &U) -> Option<T> {
        match self.pos(fencer) {
            TuplePos::First => Some(self.0.clone()),
            TuplePos::Second => Some(self.1.clone()),
            TuplePos::None => None,
        }
    }

    pub(crate) fn pos(&self, fencer: &U) -> TuplePos {
        if fencer == self.0.borrow() {
            TuplePos::First
        } else if fencer == self.1.borrow() {
            TuplePos::Second
        } else {
            TuplePos::None
        }
    }
}

impl<U: Fencer, T: Borrow<U>> Hash for FencerVs<U, T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let (a, b) = (self.0.borrow(), self.1.borrow());
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        a.hash(state);
        b.hash(state);
    }
}

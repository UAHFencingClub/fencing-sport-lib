use crate::{fencer::Fencer, pools::PoolSheetError};
use std::{
    borrow::Borrow,
    hash::{self, Hash},
    marker::PhantomData,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum TuplePos {
    First,
    Second,
    None,
}

#[derive(Debug, Eq, Clone)]
pub struct FencerVs<U: Fencer, T: Borrow<U> + Clone>(pub T, pub T, PhantomData<U>);

impl<U: Fencer, T: Borrow<U> + Clone> FencerVs<U, T> {
    pub fn new(fencer_a: T, fencer_b: T) -> Result<Self, PoolSheetError> {
        if fencer_a.borrow() == fencer_b.borrow() {
            Err(PoolSheetError::InvalidBout)
        } else {
            Ok(FencerVs(fencer_a, fencer_b, PhantomData))
        }
    }

    pub fn get_fencer<A: Borrow<U>>(&self, fencer: &A) -> Option<T> {
        match self.pos(fencer.borrow()) {
            TuplePos::First => Some(self.0.clone()),
            TuplePos::Second => Some(self.1.clone()),
            TuplePos::None => None,
        }
    }

    pub(crate) fn pos<A: Borrow<U>>(&self, fencer: &A) -> TuplePos {
        if fencer.borrow() == self.0.borrow() {
            TuplePos::First
        } else if fencer.borrow() == self.1.borrow() {
            TuplePos::Second
        } else {
            TuplePos::None
        }
    }

    fn order(&self) -> (&U, &U) {
        let (a, b) = (self.0.borrow(), self.1.borrow());
        if a <= b {
            (a, b)
        } else {
            (b, a)
        }
    }
}

impl<U: Fencer, T: Borrow<U> + Clone> Hash for FencerVs<U, T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let (a, b) = self.order();
        a.hash(state);
        b.hash(state);
    }
}

impl<U: Fencer, T: Borrow<U> + Clone> PartialEq for FencerVs<U, T> {
    fn eq(&self, other: &Self) -> bool {
        let (self_a, self_b) = self.order();
        let (other_a, other_b) = other.order();
        (self_a == other_a) & (self_b == other_b)
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash, Hasher};

    use crate::fencer::SimpleFencer;

    use super::FencerVs;

    #[test]
    fn hash_unordered_test() {
        let fencer_a = SimpleFencer::new("a");
        let fencer_b = SimpleFencer::new("b");

        let vs_ab: FencerVs<SimpleFencer, &SimpleFencer> =
            FencerVs::new(&fencer_a, &fencer_b).unwrap();

        let vs_ba: FencerVs<SimpleFencer, &SimpleFencer> =
            FencerVs::new(&fencer_b, &fencer_a).unwrap();

        let mut hash_ab = DefaultHasher::new();
        vs_ab.hash(&mut hash_ab);

        let mut hash_ba = DefaultHasher::new();
        vs_ba.hash(&mut hash_ba);

        assert_eq!(hash_ab.finish(), hash_ba.finish());
    }

    #[test]
    fn eq_unordered_test() {
        let fencer_a = SimpleFencer::new("a");
        let fencer_b = SimpleFencer::new("b");

        let vs_ab: FencerVs<SimpleFencer, &SimpleFencer> =
            FencerVs::new(&fencer_a, &fencer_b).unwrap();

        let vs_ba: FencerVs<SimpleFencer, &SimpleFencer> =
            FencerVs::new(&fencer_b, &fencer_a).unwrap();

        assert_eq!(vs_ab, vs_ba);
    }
}

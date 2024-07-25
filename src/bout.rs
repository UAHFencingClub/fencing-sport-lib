use std::{
    borrow::Borrow,
    fmt,
    hash::{self, Hash},
    marker::PhantomData,
};

use crate::cards::Cards;
use crate::fencer::Fencer;

#[derive(Debug)]
pub struct FencerScore<U: Fencer, T: Borrow<U>> {
    pub fencer: T,
    pub score: u8,
    pub cards: Cards,
    _p: PhantomData<U>,
}

#[derive(Debug)]
pub struct Bout<U: Fencer, T: Borrow<U>> {
    fencers: FencerVs<U, T>,
    scores: Option<(FencerScore<U, T>, FencerScore<U, T>)>,
}

impl<U: Fencer, T: Borrow<U>> Bout<U, T> {
    pub fn update_score(&mut self, score_a: FencerScore<U, T>, score_b: FencerScore<U, T>) {
        self.scores = Some((score_a, score_b));
    }

    pub fn new(fencers: FencerVs<U, T>) -> Self {
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

impl fmt::Display for FencerVsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FencerVsError::SameFencer => write!(f, "A fencer cannot fence themselves."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FencerVs<U: Fencer, T: Borrow<U>>(T, T, PhantomData<U>);

impl<U: Fencer, T: Borrow<U>> FencerVs<U, T> {
    pub fn new(fencer_a: T, fencer_b: T) -> Result<Self, FencerVsError> {
        if fencer_a.borrow() == fencer_b.borrow() {
            return Err(FencerVsError::SameFencer);
        }
        Ok(FencerVs(fencer_a, fencer_b, PhantomData))
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::fencer::SimpleFencer;

    use super::{Bout, FencerVs};

    #[test]
    fn bout_owned_test() {
        let fencer_a = SimpleFencer::new("Alice");
        let fencer_b = SimpleFencer::new("Bob");

        let versus: FencerVs<SimpleFencer, SimpleFencer> =
            FencerVs::new(fencer_a, fencer_b).unwrap();
        let bout = Bout::new(versus);

        assert_eq!(
            format!("{bout:?}"),
            r#"Bout { fencers: FencerVs(SimpleFencer { name: "Alice", clubs: [] }, SimpleFencer { name: "Bob", clubs: [] }, PhantomData<fencing_sport_lib::fencer::SimpleFencer>), scores: None }"#
        );
    }

    #[test]
    fn bout_box_test() {
        let fencer_a = Box::new(SimpleFencer::new("Alice"));
        let fencer_b = Box::new(SimpleFencer::new("Bob"));

        let versus: FencerVs<SimpleFencer, Box<SimpleFencer>> =
            FencerVs::new(fencer_a, fencer_b).unwrap();
        let bout = Bout::new(versus);

        assert_eq!(
            format!("{bout:?}"),
            r#"Bout { fencers: FencerVs(SimpleFencer { name: "Alice", clubs: [] }, SimpleFencer { name: "Bob", clubs: [] }, PhantomData<fencing_sport_lib::fencer::SimpleFencer>), scores: None }"#
        );
    }

    #[test]
    fn bout_rc_test() {
        let fencer_a = Rc::new(SimpleFencer::new("Alice"));
        let fencer_b = Rc::new(SimpleFencer::new("Bob"));

        let versus: FencerVs<SimpleFencer, Rc<SimpleFencer>> =
            FencerVs::new(fencer_a, fencer_b).unwrap();
        let bout = Bout::new(versus);

        assert_eq!(
            format!("{bout:?}"),
            r#"Bout { fencers: FencerVs(SimpleFencer { name: "Alice", clubs: [] }, SimpleFencer { name: "Bob", clubs: [] }, PhantomData<fencing_sport_lib::fencer::SimpleFencer>), scores: None }"#
        );
    }
}

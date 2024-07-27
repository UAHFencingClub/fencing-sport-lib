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

#[derive(Debug)]
pub struct Bout<U: Fencer, T: Borrow<U>> {
    fencers: FencerVs<U, T>,
    scores: Option<(u8, u8)>,
}

impl<U: Fencer, T: Borrow<U> + Clone> Bout<U, T> {
    pub fn update_score<S: Borrow<U>>(
        &mut self,
        score_a: FencerScore<U, S>,
        score_b: FencerScore<U, S>,
    ) -> Result<(), ()> {
        let pos_a = self.fencers.pos(score_a.fencer.borrow());
        let pos_b = self.fencers.pos(score_b.fencer.borrow());
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

        if pos_b == TuplePos::None {
            return Err(());
        }

        self.scores = Some((score_0, score_1));

        Ok(())
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

#[derive(Debug, PartialEq, Eq)]
enum TuplePos {
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

    fn pos(&self, fencer: &U) -> TuplePos {
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

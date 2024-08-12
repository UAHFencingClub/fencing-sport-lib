use std::{borrow::Borrow, cmp::Ordering};

use crate::{cards::Cards, fencer::Fencer, pools::PoolSheetError};

mod score;
pub use score::FencerScore;
mod winner;
pub use winner::BoutWinner;

mod versus;
pub use versus::FencerVs;
pub(crate) use versus::TuplePos;

#[derive(Debug, Clone, PartialEq)]
pub struct Bout<U: Fencer, T: Borrow<U> + Clone> {
    pub(crate) fencers: FencerVs<U, T>,
    pub(crate) scores: (Option<u8>, Option<u8>),
    pub(crate) cards: (Cards, Cards),
    pub(crate) priority: TuplePos,
}

impl<U: Fencer, T: Borrow<U> + Clone> Bout<U, T> {
    pub fn update_scores<S: Borrow<U>>(
        &mut self,
        score_a: FencerScore<U, S>,
        score_b: FencerScore<U, S>,
    ) -> Result<(), PoolSheetError> {
        let pos_a = self.fencers.pos(score_a.fencer.borrow());
        let pos_b = self.fencers.pos(score_b.fencer.borrow());
        if pos_a == pos_b {
            return Err(PoolSheetError::InvalidBout);
        }

        let score_0;
        let score_1;
        let cards_0;
        let cards_1;

        match pos_a {
            TuplePos::First => {
                score_0 = score_a.score;
                score_1 = score_b.score;
                cards_0 = score_a.cards;
                cards_1 = score_b.cards;
            }
            TuplePos::Second => {
                score_1 = score_a.score;
                score_0 = score_b.score;
                cards_1 = score_a.cards;
                cards_0 = score_b.cards;
            }
            TuplePos::None => return Err(PoolSheetError::InvalidBout),
        }

        if pos_b == TuplePos::None {
            return Err(PoolSheetError::InvalidBout);
        }

        self.scores = (Some(score_0), Some(score_1));
        self.cards = (cards_0, cards_1);
        Ok(())
    }

    pub fn get_scores(&self) -> Option<(u8, u8)> {
        self.scores.0.zip(self.scores.1)
    }

    pub fn set_score<V: Borrow<U>>(
        &mut self,
        fencer_score: FencerScore<U, V>,
    ) -> Result<(), PoolSheetError> {
        match self.fencers.pos(fencer_score.fencer.borrow()) {
            TuplePos::First => {
                self.scores.0 = Some(fencer_score.score);
                self.cards.0 = fencer_score.cards;
            }
            TuplePos::Second => {
                self.scores.1 = Some(fencer_score.score);
                self.cards.1 = fencer_score.cards;
            }
            TuplePos::None => return Err(PoolSheetError::InvalidBout),
        }
        Ok(())
    }

    pub fn get_score<V: Borrow<U>>(&self, fencer: V) -> Option<u8> {
        match self.fencers.pos(fencer.borrow()) {
            TuplePos::First => self.scores.0,
            TuplePos::Second => self.scores.1,
            TuplePos::None => None,
        }
    }

    pub fn unset_score<V: Borrow<U>>(&mut self, fencer: V) -> Result<(), PoolSheetError> {
        match self.fencers.pos(fencer.borrow()) {
            TuplePos::First => self.scores.0 = None,
            TuplePos::Second => self.scores.1 = None,
            TuplePos::None => return Err(PoolSheetError::InvalidBout),
        }
        Ok(())
    }

    pub fn unset_scores(&mut self) {
        self.scores = (None, None);
    }

    pub fn get_fencers(&self) -> (&U, &U) {
        (self.fencers.0.borrow(), self.fencers.1.borrow())
    }

    pub fn get_fencers_owned(&self) -> (T, T) {
        self.fencers.get_fencers_owned()
    }

    pub fn set_priority<V: Borrow<U>>(&mut self, fencer: Option<V>) -> Result<(), PoolSheetError> {
        match fencer {
            Some(fencer) => {
                let pos = self.fencers.pos(fencer.borrow());
                if pos == TuplePos::None {
                    Err(PoolSheetError::InvalidBout)
                } else {
                    self.priority = pos;
                    Ok(())
                }
            }
            None => {
                self.priority = TuplePos::None;
                Ok(())
            }
        }
    }

    pub fn get_priority(&self) -> Option<&U> {
        match self.priority {
            TuplePos::First => Some(self.fencers.0.borrow()),
            TuplePos::Second => Some(self.fencers.1.borrow()),
            TuplePos::None => None,
        }
    }

    pub fn get_winner(&self) -> Option<&U> {
        let (score_a, score_b) = self.scores.0.zip(self.scores.1)?;
        match score_a.cmp(&score_b) {
            Ordering::Greater => Some(self.fencers.0.borrow()),
            Ordering::Less => Some(self.fencers.1.borrow()),
            Ordering::Equal => self.get_priority(),
        }
    }

    pub fn new(fencers: FencerVs<U, T>) -> Self {
        Bout {
            fencers,
            scores: (None, None),
            cards: (Cards::default(), Cards::default()),
            priority: TuplePos::None,
        }
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

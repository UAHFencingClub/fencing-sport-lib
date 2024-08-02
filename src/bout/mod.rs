use std::borrow::Borrow;

use crate::{fencer::Fencer, pools::PoolSheetError};

mod score;
pub use score::FencerScore;
mod winner;
pub use winner::BoutWinner;

mod versus;
pub use versus::FencerVs;
use versus::TuplePos;

#[derive(Debug)]
pub struct Bout<U: Fencer, T: Borrow<U> + Clone> {
    pub(crate) fencers: FencerVs<U, T>,
    pub(crate) scores: Option<(u8, u8)>,
    pub(crate) winner: Option<T>,
}

impl<U: Fencer, T: Borrow<U> + Clone> Bout<U, T> {
    pub fn update_score<S: Borrow<U>>(
        &mut self,
        score_a: FencerScore<U, S>,
        score_b: FencerScore<U, S>,
        winner: BoutWinner<U, S>,
    ) -> Result<(), PoolSheetError> {
        let pos_a = self.fencers.pos(score_a.fencer.borrow());
        let pos_b = self.fencers.pos(score_b.fencer.borrow());
        if pos_a == pos_b {
            return Err(PoolSheetError::InvalidBout);
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
            TuplePos::None => return Err(PoolSheetError::InvalidBout),
        }

        if pos_b == TuplePos::None {
            return Err(PoolSheetError::InvalidBout);
        }

        self.scores = Some((score_0, score_1));

        match winner {
            BoutWinner::Auto(_) => {
                if score_0 > score_1 {
                    self.winner = Some(self.fencers.0.clone())
                } else if score_1 > score_0 {
                    self.winner = Some(self.fencers.1.clone())
                } else {
                    return Err(PoolSheetError::UnableToCompleteBout_REEVALUATE);
                }
            }
            BoutWinner::Manual(winner) => todo!(),
        }

        Ok(())
    }

    pub fn get_fencers(&self) -> (&U, &U) {
        (self.fencers.0.borrow(), self.fencers.1.borrow())
    }

    pub fn get_scores(&self) -> Option<(u8, u8)> {
        self.scores
    }

    pub fn get_score<V: Borrow<U>>(&self, fencer: V) -> Option<u8> {
        match self.fencers.pos(fencer.borrow()) {
            TuplePos::First => self.scores.map(|x| x.0),
            TuplePos::Second => self.scores.map(|x| x.1),
            TuplePos::None => None,
        }
    }

    pub fn new(fencers: FencerVs<U, T>) -> Self {
        Bout {
            fencers,
            scores: None,
            winner: None,
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

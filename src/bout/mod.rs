use std::borrow::Borrow;

use crate::fencer::Fencer;

mod score;
pub use score::FencerScore;

mod versus;
use versus::TuplePos;
pub use versus::{FencerVs, FencerVsError};

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

use derive_getters::Getters;
use indexmap::{map::Iter, IndexMap};
use iter_tools::Itertools;

use crate::fencer::Fencer;
use std::fmt::Debug;
use std::{borrow::Borrow, cmp::Ordering, rc::Rc};

use super::Placement;
use super::PoolSheet;

use rand::random;

#[derive(Debug, Clone, Getters)]
pub struct FencerResult<T: Fencer> {
    fencer: Rc<T>,
    victories: u8,
    touches_scored: u8,
    touches_recieved: u8,
    indicator: i16,
    place: Placement,
}

impl<T: Fencer> FencerResult<T> {
    fn new_zeroed(fencer: Rc<T>) -> FencerResult<T> {
        FencerResult {
            fencer: fencer.clone(),
            victories: 0,
            touches_scored: 0,
            touches_recieved: 0,
            indicator: 0,
            place: Placement::Absolute(0),
        }
    }

    fn calculate_indicator(&mut self) {
        self.indicator = i16::from(self.touches_scored) - i16::from(self.touches_recieved);
    }

    /// Compares result, choosing one at random if they are equal.
    fn rnd_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other).then_with(|| {
            if random::<bool>() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
    }
}

impl<T: Fencer> PartialEq for FencerResult<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.victories == other.victories) & (self.indicator == other.indicator)
    }
}

/// Validate usage
impl<T: Fencer> Eq for FencerResult<T> {}

impl<T: Fencer> PartialOrd for FencerResult<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Fencer> Ord for FencerResult<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.victories
            .cmp(&other.victories)
            .then_with(|| self.indicator.cmp(&other.indicator))
    }
}

#[derive(Debug, Clone)]
pub struct PoolResults<T: Fencer>(IndexMap<Rc<T>, FencerResult<T>>);

impl<T: Fencer + Debug> PoolResults<T> {
    pub fn new(poolsheet: &PoolSheet<T>) -> PoolResults<T> {
        let mut results_map = IndexMap::new();
        for fencer in poolsheet.fencers.iter() {
            results_map.insert(fencer.clone(), FencerResult::new_zeroed(fencer.clone()));
        }

        for (_, bout) in poolsheet.bouts.iter() {
            let (fencer_a, fencer_b) = (&bout.fencers.0, &bout.fencers.1);
            let (score_a, score_b) = bout
                .get_scores()
                .expect("Bout Should have been checked for completion");

            let bout_winner = bout
                .get_winner()
                .expect("Winner should be set before a call to make results is done");

            {
                let fencer_a_result = results_map
                    .get_mut(fencer_a)
                    .expect("The map should be populated with all possible fencers");

                fencer_a_result.touches_scored += score_a;
                fencer_a_result.touches_recieved += score_b;

                if <Rc<T> as Borrow<T>>::borrow(&(*fencer_a)) == bout_winner {
                    fencer_a_result.victories += 1
                }
            }
            {
                let fencer_b_result = results_map
                    .get_mut(fencer_b)
                    .expect("The map should be populated with all possible fencers");

                fencer_b_result.touches_scored += score_b;
                fencer_b_result.touches_recieved += score_a;

                if <Rc<T> as Borrow<T>>::borrow(&(*fencer_b)) == bout_winner {
                    fencer_b_result.victories += 1
                }
            }
        }

        for (_, fencer_result) in results_map.iter_mut() {
            fencer_result.calculate_indicator();
        }

        results_map.sort_by(|_, a, _, b| b.rnd_cmp(a));

        let first_result = results_map
            .get_index_mut(0)
            .expect("Should have at least one fencer")
            .1;
        first_result.place = Placement::Absolute(1);

        let mut results_iter = results_map.iter_mut();

        let (_, mut last_result) = results_iter
            .next()
            .expect("Should have at least one fencer");
        for (_, result) in results_iter {
            match last_result.cmp(&result) {
                Ordering::Greater => {
                    result.place = Placement::Absolute(last_result.place.inner() + 1)
                }
                Ordering::Equal => {
                    last_result.place.to_tied();
                    result.place = Placement::Tied(last_result.place.inner())
                }
                Ordering::Less => {
                    panic!("The results should have been sorted!")
                }
            }
            last_result = result;
        }

        PoolResults(results_map)
    }

    pub fn iter(&self) -> Iter<Rc<T>, FencerResult<T>> {
        self.0.iter()
    }
}

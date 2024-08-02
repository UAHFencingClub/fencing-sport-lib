use indexmap::{map::Iter, IndexMap};

use crate::fencer::Fencer;
use std::{cmp::Ordering, rc::Rc};

use super::PoolSheet;

#[derive(Debug, Clone)]
pub struct FencerResult<T: Fencer> {
    fencer: Rc<T>,
    victories: u8,
    touches_scored: u8,
    touches_recieved: u8,
    indicator: i16,
    place: u8,
}

impl<T: Fencer> FencerResult<T> {
    fn new_zeroed(fencer: Rc<T>) -> FencerResult<T> {
        FencerResult {
            fencer: fencer.clone(),
            victories: 0,
            touches_scored: 0,
            touches_recieved: 0,
            indicator: 0,
            place: 0,
        }
    }
    fn calculate_indicator(&mut self) {
        self.indicator = i16::from(self.touches_scored) - i16::from(self.touches_recieved);
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
        Some(
            self.victories
                .cmp(&other.victories)
                .then_with(|| self.indicator.cmp(&other.indicator)),
        )
    }
}

impl<T: Fencer> Ord for FencerResult<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct PoolResults<T: Fencer>(IndexMap<Rc<T>, FencerResult<T>>);

impl<T: Fencer> PoolResults<T> {
    pub fn new(poolsheet: &PoolSheet<T>) -> PoolResults<T> {
        let mut results_map = IndexMap::new();
        for fencer in poolsheet.fencers.into_iter() {
            results_map.insert(fencer.clone(), FencerResult::new_zeroed(fencer.clone()));
        }

        for (_, bout) in poolsheet.bouts.iter() {
            let (fencer_a, fencer_b) = (&bout.fencers.0, &bout.fencers.1);
            let (score_a, score_b) = bout
                .get_scores()
                .expect("Bout Should have been checked for completion");

            let bout_winner = if score_a > score_b {
                fencer_a
            } else if score_b > score_a {
                fencer_b
            } else {
                todo!("I totally forgot to implement that functionality (bout won by priority)")
            };

            {
                let fencer_a_result = results_map
                    .get_mut(fencer_a)
                    .expect("The map should be populated with all possible fencers");

                fencer_a_result.touches_scored += score_a;
                fencer_a_result.touches_recieved += score_b;

                if fencer_a == bout_winner {
                    fencer_a_result.victories += 1
                }
            }
            {
                let fencer_b_result = results_map
                    .get_mut(fencer_b)
                    .expect("The map should be populated with all possible fencers");

                fencer_b_result.touches_scored += score_b;
                fencer_b_result.touches_recieved += score_a;

                if fencer_b == bout_winner {
                    fencer_b_result.victories += 1
                }
            }
        }

        for (_, fencer_result) in results_map.iter_mut() {
            fencer_result.calculate_indicator();
        }

        results_map.sort_by(|_, a, _, b| b.cmp(a));
        PoolResults(results_map)
    }

    pub fn iter(&self) -> Iter<Rc<T>, FencerResult<T>> {
        self.0.iter()
    }
}

use std::collections::hash_map::RandomState;

use indexmap::IndexMap;

use crate::fencer::Fencer;
use crate::bout::{Bout, FencerVs, FencerVsError};
use crate::organizations::pool_bout_orders::{self, PoolOrderError};

#[derive(Debug)]
enum BoutCreationError {
    VsError(FencerVsError, String),
    PoolOrderError(PoolOrderError),
}

trait BoutsCreator {
    fn get_order(&self, fencers: &Vec<Fencer>) -> Result<Vec<(usize, usize)>, PoolOrderError>;
}

struct SimpleBoutsCreator;

impl BoutsCreator for SimpleBoutsCreator {
    fn get_order(&self, fencers: &Vec<Fencer>) -> Result<Vec<(usize, usize)>, PoolOrderError> {
        let fencer_count = fencers.len();
        pool_bout_orders::get_default_order(fencer_count)
    }
}

#[derive(Debug)]
#[derive(Default)]
// #[derive(Default)]
struct PoolSheet {
    fencers: Vec<Fencer>,
    bouts: IndexMap<FencerVs, Bout, RandomState>,
}

impl PoolSheet {
    fn add_fencer(&mut self, fencer: Fencer) {
        self.fencers.push(fencer);
    }

    fn add_fencers<I>(&mut self, fencers: I)
    where
        I: Iterator<Item = Fencer>,
    {
        self.fencers.extend(fencers);
    }

    // function definition suggested by generative AI
    fn create_bouts<C>(&mut self, creator: &C) -> Result<(), BoutCreationError>
    where
        C: BoutsCreator,
    {
        let test = &self.fencers;
        match creator.get_order(test) {
            Ok(bout_indexes) => {
                for pair in bout_indexes.into_iter() {
                    match FencerVs::new(
                        Box::new(self.fencers.get(pair.0-1).unwrap().clone()),
                        Box::new(self.fencers.get(pair.1-1).unwrap().clone())
                    ) {
                        Ok(versus) => {
                            self.bouts.insert(versus.clone(), Bout::new(versus));
                        },
                        Err(err) => {
                            return Err(BoutCreationError::VsError(err, "The pool creation paired a fencer with themselves.".to_string()))
                        }
                    }
                }
                Ok(())
            }
            Err(err) => {
                Err(BoutCreationError::PoolOrderError(err))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bout::{self, FencerVs}, fencer::Fencer};
    use super::{BoutsCreator, PoolSheet, SimpleBoutsCreator};

    #[test]
    fn iter_test() {
        let fencers = [
            Fencer::with_name("Fencer1".to_string()),
            Fencer::with_name("Fencer2".to_string()),
            Fencer::with_name("Fencer3".to_string()),
            Fencer::with_name("Fencer4".to_string()),
        ];

        let mut pool_sheet = PoolSheet::default();
        pool_sheet.add_fencers(fencers.into_iter());
        let _ = pool_sheet.create_bouts(&SimpleBoutsCreator);
        for bout in pool_sheet.bouts {
            println!("{bout:#?}");
        }
    }

    #[test]
    fn bout_addressing() {
        let fencers = [
            Fencer::with_name("Fencer1".to_string()),
            Fencer::with_name("Fencer2".to_string()),
            Fencer::with_name("Fencer3".to_string()),
            Fencer::with_name("Fencer4".to_string()),
        ];

        let mut pool_sheet = PoolSheet::default();
        pool_sheet.add_fencers(fencers.clone().into_iter());
        let _ = pool_sheet.create_bouts(&SimpleBoutsCreator);

        let a_versus = FencerVs::new(Box::new(fencers[0].clone()), Box::new(fencers[1].clone())).unwrap();

        let a_bout = pool_sheet.bouts.get_mut(&a_versus).unwrap();
        a_bout.update_score(0,5);
        println!("\nSingle Bout: {a_bout:#?}");

    }
}
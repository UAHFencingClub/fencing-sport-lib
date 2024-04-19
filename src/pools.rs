use std::collections::hash_map::RandomState;

use indexmap::IndexMap;

use crate::fencer::Fencer;
use crate::bout::{Bout, FencerVs, FencerVsError};
use crate::organizations::pool_bout_orders::{self, PoolOrderError};

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
struct PoolSheet<'b> {
    fencers: Vec<Fencer>,
    bouts: IndexMap<FencerVs<'b>,Bout<'b>, RandomState>,
    // bouts: Vec<Bout<'a>>
}

impl<'a, 'b> PoolSheet<'b> {
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
    fn create_bouts<C>(&'a mut self, creator: &C)
    where
        C: BoutsCreator,
        'a: 'b,
    {
        // let mut tmp_bouts = IndexMap::<FencerVs::<>,Bout::<>,RandomState>::new();
        match creator.get_order(&self.fencers) {
            Ok(bout_indexes) => {
                for pair in bout_indexes.into_iter() {
                    match FencerVs::<'b>::new(
                        self.fencers.get(pair.0-1).unwrap(),
                        self.fencers.get(pair.1-1).unwrap()
                    ) {
                        Ok(versus) => {
                            self.bouts.insert(versus,Bout::new(versus));
                        },
                        Err(err) => {
                            // return Err(BoutCreationError::VsError(err, "The pool creation paied a fencer with themselves.".to_string()))
                        }
                    }
                }
                // Ok(())
            }
            Err(err) => {
                // Err(BoutCreationError::PoolOrderError(err))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bout, fencer::Fencer};

    use super::{BoutsCreator, PoolSheet, SimpleBoutsCreator};

    #[test]
    fn initial_test() {
        let fencers = [
            Fencer::with_name("Fencer1".to_string()),
            Fencer::with_name("Fencer2".to_string()),
            Fencer::with_name("Fencer3".to_string()),
            Fencer::with_name("Fencer4".to_string()),
        ];

        let mut pool_sheet = PoolSheet::default();
        pool_sheet.add_fencers(fencers.into_iter());
        pool_sheet.create_bouts(&SimpleBoutsCreator);
        
        let test = &pool_sheet.bouts;
        for bout in test.into_iter() {
            let test = format!("{bout:?}");
            println!("{test}")
        }
    }
}
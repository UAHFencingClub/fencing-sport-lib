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
    fn create_bouts(&mut self, pool_sheet: &mut PoolSheet) -> Result<(), BoutCreationError>;
}

struct SimpleBoutsCreator;

impl BoutsCreator for SimpleBoutsCreator {
    fn create_bouts(&mut self, pool_sheet: &mut PoolSheet) -> Result<(), BoutCreationError> {
        let fencer_count = pool_sheet.fencers.len();
        // let mut tmp_bouts = IndexMap::<FencerVs::<'a>,Bout::<'a>,RandomState>::new();
        match pool_bout_orders::get_default_order(fencer_count) {
            Ok(bout_indexes) => {
                for pair in bout_indexes.into_iter() {
                    match FencerVs::new(
                        pool_sheet.fencers.get(pair.0-1).unwrap(),
                        pool_sheet.fencers.get(pair.1-1).unwrap()
                    ) {
                        Ok(versus) => {
                            pool_sheet.bouts.insert(versus,Bout::new(versus));
                        },
                        Err(err) => {
                            return Err(BoutCreationError::VsError(err, "The pool creation paied a fencer with themselves.".to_string()))
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

#[derive(Debug)]
// #[derive(Default)]
struct PoolSheet<'a> {
    fencers: Vec<Fencer>,
    bouts: IndexMap<FencerVs<'a>,Bout<'a>, RandomState>,
}

impl<'a> PoolSheet<'a> {
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
    fn create_bouts<C>(&mut self, creator: &mut C)
    where
        C: BoutsCreator,
    {
        creator.create_bouts(self);
    }
}

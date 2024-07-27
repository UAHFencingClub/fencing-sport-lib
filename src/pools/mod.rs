use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::rc::Rc;

use indexmap::IndexMap;

use crate::bout::{Bout, FencerScore, FencerVs, FencerVsError};
use crate::fencer::Fencer;
use crate::organizations::usafencing::pool_bout_orders::{get_default_order, PoolOrderError};
use bout_creation::{BoutCreationError, BoutsCreator};

pub mod bout_creation;

#[derive(Debug)]
pub struct PoolSheet<T: Fencer> {
    fencers: Box<[Rc<T>]>,
    bouts: IndexMap<FencerVs<T, Rc<T>>, Bout<T, Rc<T>>, RandomState>,
}

impl<T: Fencer> PoolSheet<T> {
    pub fn new<C>(fencers: Vec<T>, creator: &C) -> Result<PoolSheet<T>, PoolOrderError>
    where
        C: BoutsCreator<T>,
    {
        let mut fencers_rced = Vec::with_capacity(fencers.len());

        let bout_indexes: Vec<(usize, usize)> = creator.get_order(&fencers)?;

        for fencer in fencers {
            fencers_rced.push(Rc::new(fencer));
        }

        let mut new_sheet = PoolSheet {
            fencers: fencers_rced.into_boxed_slice(),
            bouts: IndexMap::new(),
        };

        for pair in bout_indexes.into_iter() {
            let versus = FencerVs::new(
                new_sheet.fencers.get(pair.0 - 1).unwrap().clone(),
                new_sheet.fencers.get(pair.1 - 1).unwrap().clone(),
            )
            .expect("Error in bout creator, invalid indexes generated.");
            // .map_err(|err| {
            //     BoutCreationError::VsError(
            //         err,
            //         "The pool creation paired a fencer with themselves.".to_string(),
            //     )
            // })?;

            new_sheet.bouts.insert(versus.clone(), Bout::new(versus));
        }

        Ok(new_sheet)
    }

    pub fn iter(&self) -> indexmap::map::Iter<FencerVs<T, Rc<T>>, Bout<T, Rc<T>>> {
        self.bouts.iter()
    }

    pub fn update_score(
        &mut self,
        fencer_a: FencerScore<T, T>,
        fencer_b: FencerScore<T, T>,
    ) -> Result<(), PoolOrderError> {
        let fencer_a_fencer = Rc::new(fencer_a.fencer);
        let fencer_b_fencer = Rc::new(fencer_b.fencer);

        let fencer_a = FencerScore::new()
        let x = FencerVs::new(fencer_a_fencer, fencer_b_fencer).unwrap();
        let bout = self.bouts.get_mut(&x).unwrap();
        bout.update_score();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{bout_creation::SimpleBoutsCreator, PoolSheet};
    use crate::{
        bout::{FencerScore, FencerVs},
        cards::Cards,
        fencer::SimpleFencer,
    };

    #[test]
    fn iter_test() {
        let fencers = [
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];

        let mut pool_sheet = PoolSheet::default();
        pool_sheet.add_fencers(fencers.into_iter());
        let _ = pool_sheet.create_bouts(&SimpleBoutsCreator);
        for bout in pool_sheet.iter() {
            println!("{bout:#?}");
        }
    }

    #[test]
    fn bout_addressing() {
        let fencers = [
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];

        let json_fencer1 =
            serde_json::from_str::<SimpleFencer>(r#"{"name":"Fencer1","clubs":[]}"#).unwrap();
        let json_fencer2 =
            serde_json::from_str::<SimpleFencer>(r#"{"name":"Fencer2","clubs":[]}"#).unwrap();

        let mut pool_sheet = PoolSheet::default();
        pool_sheet.add_fencers(fencers.clone().into_iter());
        let _ = pool_sheet.create_bouts(&SimpleBoutsCreator);

        let a_versus = FencerVs::new(json_fencer1.clone(), json_fencer2.clone()).unwrap();

        let smth = pool_sheet.update_score(
            FencerScore {
                fencer: json_fencer1,
                score: 0,
                cards: Cards::default(),
            },
            FencerScore {
                fencer: json_fencer2,
                score: 0,
                cards: Cards::default(),
            },
        );
        println!("\nSingle Bout: {smth:#?}");
    }
}

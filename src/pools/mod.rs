use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::marker::PhantomData;
use std::rc::Rc;

use indexmap::IndexMap;

use crate::bout::{Bout, FencerScore, FencerVs, FencerVsError};
use crate::fencer::{self, Fencer};
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
    ) -> Result<(), ()> {
        let fencer_a_fencer = Rc::new(fencer_a.fencer);
        let fencer_b_fencer = Rc::new(fencer_b.fencer);

        let fencer_a: FencerScore<T, Rc<T>> =
            FencerScore::new(fencer_a_fencer.clone(), fencer_a.score, fencer_a.cards);
        let fencer_b: FencerScore<T, Rc<T>> =
            FencerScore::new(fencer_b_fencer.clone(), fencer_b.score, fencer_b.cards);

        let x = FencerVs::new(fencer_a.fencer, fencer_b.fencer).unwrap();
        let (_, vs, bout) = self.bouts.get_full_mut(&x).unwrap();

        let fencer_a = FencerScore::new(
            vs.get_fencer(&fencer_a_fencer).unwrap(),
            fencer_a.score,
            fencer_a.cards,
        );

        let fencer_b = FencerScore::new(
            vs.get_fencer(&fencer_b_fencer).unwrap(),
            fencer_b.score,
            fencer_b.cards,
        );

        let acs = Rc::strong_count(&fencer_a_fencer);
        let bcs = Rc::strong_count(&fencer_b_fencer);
        let acw = Rc::weak_count(&fencer_a_fencer);
        let bcw = Rc::weak_count(&fencer_b_fencer);

        println!("Counts {}, {}, {}, {}", acs, bcs, acw, bcw);

        bout.update_score(fencer_a, fencer_b)
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

        let pool_sheet = PoolSheet::new(fencers.to_vec(), &SimpleBoutsCreator).unwrap();
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

        let mut pool_sheet = PoolSheet::new(fencers.to_vec(), &SimpleBoutsCreator).unwrap();

        let _smth = pool_sheet.update_score(
            FencerScore::new(json_fencer1, 0, Cards::default()),
            FencerScore::new(json_fencer2, 0, Cards::default()),
        );
        println!("\nSingle Bout: {pool_sheet:#?}");
    }
}

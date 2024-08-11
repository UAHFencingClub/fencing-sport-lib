use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::marker::PhantomData;
use std::rc::Rc;

use indexmap::map::Iter;
use indexmap::{IndexMap, IndexSet};
pub use result::PoolResults;
use serde::Serialize;

use crate::bout::{Bout, FencerScore, FencerVs};
use crate::fencer::Fencer;
use bout_creation::BoutsCreator;

pub mod bout_creation;
mod pool_error;
pub use pool_error::PoolSheetError;
pub mod result;

pub type PoolSheetFencerScore<T> = FencerScore<T, Rc<T>>;
pub type PoolSheetVersus<T> = FencerVs<T, Rc<T>>;
pub type PoolSheetBout<T> = Bout<T, Rc<T>>;
pub type PoolBoutIter<'a, T> = Iter<'a, FencerVs<T, Rc<T>>, Bout<T, Rc<T>>>;

#[derive(Debug, Clone)]
pub struct PoolSheet<T: Fencer> {
    fencers: Box<[Rc<T>]>,
    bouts: IndexMap<PoolSheetVersus<T>, PoolSheetBout<T>, RandomState>,
}

impl<T: Fencer> PoolSheet<T> {
    pub fn new<C>(fencers: IndexSet<T>, creator: &C) -> Result<PoolSheet<T>, PoolSheetError>
    where
        C: BoutsCreator<T>,
    {
        let fencers: Vec<T> = fencers.into_iter().collect();
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
                new_sheet.fencers[pair.0 - 1].clone(),
                new_sheet.fencers[pair.1 - 1].clone(),
            )
            .expect("Error in bout creator, invalid indexes generated.");

            new_sheet.bouts.insert(versus.clone(), Bout::new(versus));
        }

        Ok(new_sheet)
    }

    pub fn get_fencers(&self) -> Vec<&T> {
        self.fencers.as_ref().iter().map(|x| x.as_ref()).collect()
    }

    pub fn get_bout<U: Borrow<T> + Clone + Eq>(
        &self,
        vs: &FencerVs<T, U>,
    ) -> Result<&PoolSheetBout<T>, PoolSheetError> {
        let a = Rc::new(vs.0.borrow().clone());
        let b = Rc::new(vs.1.borrow().clone());
        let vs = FencerVs::new(a, b).unwrap();
        self.bouts.get(&vs).ok_or(PoolSheetError::NoBoutFound)
    }

    pub fn get_bout_mut<U: Borrow<T> + Clone + Eq>(
        &mut self,
        vs: &FencerVs<T, U>,
    ) -> Result<&mut PoolSheetBout<T>, PoolSheetError> {
        let a = Rc::new(vs.0.borrow().clone());
        let b = Rc::new(vs.1.borrow().clone());
        let vs = FencerVs::new(a, b).unwrap();
        self.bouts.get_mut(&vs).ok_or(PoolSheetError::NoBoutFound)
    }

    pub fn iter_bouts(&self) -> indexmap::map::Iter<FencerVs<T, Rc<T>>, Bout<T, Rc<T>>> {
        self.bouts.iter()
    }

    pub fn update_score<U: Borrow<T> + Clone + Eq>(
        &mut self,
        fencer_a: FencerScore<T, U>,
        fencer_b: FencerScore<T, U>,
    ) -> Result<(), PoolSheetError> {
        // Need to convert fencerscore struct since the index map needs a version using an Rc smart pointer.
        // This does mean calling this function requires 2 heap allocations every time it is used
        // I did do some earlier testing in previous commits to make sure that the data is dropped after this function call
        let fencer_a_fencer = Rc::new(fencer_a.fencer.borrow().clone());
        let fencer_b_fencer = Rc::new(fencer_b.fencer.borrow().clone());

        let x = FencerVs::new(fencer_a_fencer, fencer_b_fencer)?;
        let (_, vs, bout) = self
            .bouts
            .get_full_mut(&x)
            .ok_or(PoolSheetError::NoBoutFound)?;

        let fencer_a = FencerScore::new(
            vs.get_fencer(&fencer_a.fencer)
                .expect("This should have been checked earlier in the function"),
            fencer_a.score,
            fencer_a.cards,
        );

        let fencer_b = FencerScore::new(
            vs.get_fencer(&fencer_b.fencer)
                .expect("This should have been checked earlier in the function"),
            fencer_b.score,
            fencer_b.cards,
        );

        bout.update_scores(fencer_a, fencer_b)
    }

    pub fn unset_score<U: Borrow<T> + Clone + Eq>(
        &mut self,
        fencer_a: FencerScore<T, U>,
        fencer_b: FencerScore<T, U>,
    ) -> Result<(), PoolSheetError> {
        // Need to convert fencerscore struct since the index map needs a version using an Rc smart pointer.
        // This does mean calling this function requires 2 heap allocations every time it is used
        // I did do some earlier testing in previous commits to make sure that the data is dropped after this function call
        let fencer_a_fencer = Rc::new(fencer_a.fencer.borrow().clone());
        let fencer_b_fencer = Rc::new(fencer_b.fencer.borrow().clone());

        let x = FencerVs::new(fencer_a_fencer, fencer_b_fencer)?;
        let (_, _, bout) = self
            .bouts
            .get_full_mut(&x)
            .ok_or(PoolSheetError::NoBoutFound)?;

        bout.unset_scores();
        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        for (_, bout) in self.bouts.iter() {
            if bout.get_winner().is_none() {
                return false;
            }
        }
        true
    }

    pub fn finish(&self) -> Result<PoolResults<T>, PoolSheetError> {
        if self.is_finished() {
            Ok(PoolResults::new(self))
        } else {
            Err(PoolSheetError::PoolNotComplete)
        }
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexSet;

    use super::{bout_creation::SimpleBoutsCreator, PoolSheet};
    use crate::{bout::FencerScore, cards::Cards, fencer::SimpleFencer};

    #[test]
    fn from_vec_test() {
        let fencers = vec![
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];

        let _pool_sheet =
            PoolSheet::new(IndexSet::from_iter(fencers), &SimpleBoutsCreator).unwrap();
    }

    #[test]
    fn iter_test() {
        let fencers = [
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];

        let pool_sheet = PoolSheet::new(fencers.into(), &SimpleBoutsCreator).unwrap();
        for bout in pool_sheet.iter_bouts() {
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

        let mut pool_sheet = PoolSheet::new(fencers.into(), &SimpleBoutsCreator).unwrap();

        let _smth = pool_sheet.update_score(
            FencerScore::new(json_fencer1, 0, Cards::default()),
            FencerScore::new(json_fencer2, 0, Cards::default()),
        );
        println!("\nSingle Bout: {pool_sheet:#?}");
    }

    #[test]
    fn update_score_unordered() {
        // Make sure that the order of inputting scores does not matter.
        todo!();
    }
}

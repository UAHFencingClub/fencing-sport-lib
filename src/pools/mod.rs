use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::rc::Rc;

use indexmap::map::Iter;
use indexmap::IndexMap;

use crate::bout::{Bout, FencerScore, FencerVs};
use crate::fencer::Fencer;
use bout_creation::BoutsCreator;

pub mod bout_creation;
mod pool_error;
pub use pool_error::PoolSheetError;

type PoolSheetVersus<T> = FencerVs<T, Rc<T>>;
pub type PoolSheetBout<T> = Bout<T, Rc<T>>;
pub type PoolBoutIter<'a, T> = Iter<'a, FencerVs<T, Rc<T>>, Bout<T, Rc<T>>>;

#[derive(Debug)]
pub struct PoolSheet<T: Fencer> {
    fencers: Box<[Rc<T>]>,
    bouts: IndexMap<PoolSheetVersus<T>, PoolSheetBout<T>, RandomState>,
}

impl<T: Fencer> PoolSheet<T> {
    pub fn new<C>(fencers: Vec<T>, creator: &C) -> Result<PoolSheet<T>, PoolSheetError>
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

    pub fn iter_bouts(&self) -> indexmap::map::Iter<FencerVs<T, Rc<T>>, Bout<T, Rc<T>>> {
        self.bouts.iter()
    }

    pub fn update_score(
        &mut self,
        fencer_a: FencerScore<T, T>,
        fencer_b: FencerScore<T, T>,
    ) -> Result<(), PoolSheetError> {
        // Formatted weirdly to do some tests to make sure the new smart pointer gets dropped.
        let buf;
        // let testa;
        // let testb;
        {
            // Need to convert fencerscore struct since the index map needs a version using an Rc smart pointer.
            // I put it in this block so I test and make sure that I dont end up with additional references to the new pointer
            // by passing it into the bout. These instances should only exist for this function.
            let fencer_a_fencer = Rc::new(fencer_a.fencer.clone());
            let fencer_b_fencer = Rc::new(fencer_b.fencer.clone());

            let fencer_a: FencerScore<T, Rc<T>> =
                FencerScore::new(fencer_a_fencer.clone(), fencer_a.score, fencer_a.cards);
            let fencer_b: FencerScore<T, Rc<T>> =
                FencerScore::new(fencer_b_fencer.clone(), fencer_b.score, fencer_b.cards);

            let x = FencerVs::new(fencer_a.fencer, fencer_b.fencer)?;
            buf = self
                .bouts
                .get_full_mut(&x)
                .ok_or(PoolSheetError::NoBoutFound)?;

            // testa = fencer_a_fencer;
            // testb = fencer_b_fencer;
        }

        // let acs = Rc::strong_count(&testa);
        // let bcs = Rc::strong_count(&testb);
        // let acw = Rc::weak_count(&testa);
        // let bcw = Rc::weak_count(&testb);

        // I only expect 1 strong count for each since.
        // println!("Counts {}, {}, {}, {}", acs, bcs, acw, bcw);

        let (_, vs, bout) = buf;

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

        bout.update_score(fencer_a, fencer_b)
    }
}

#[cfg(test)]
mod tests {
    use super::{bout_creation::SimpleBoutsCreator, PoolSheet};
    use crate::{bout::FencerScore, cards::Cards, fencer::SimpleFencer};

    #[test]
    fn iter_test() {
        let fencers = [
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];

        let pool_sheet = PoolSheet::new(fencers.to_vec(), &SimpleBoutsCreator).unwrap();
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

        let mut pool_sheet = PoolSheet::new(fencers.to_vec(), &SimpleBoutsCreator).unwrap();

        let _smth = pool_sheet.update_score(
            FencerScore::new(json_fencer1, 0, Cards::default()),
            FencerScore::new(json_fencer2, 0, Cards::default()),
        );
        println!("\nSingle Bout: {pool_sheet:#?}");
    }
}

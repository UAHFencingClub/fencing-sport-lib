pub mod bout_creation;

use ouroboros::self_referencing;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;

use indexmap::IndexMap;

use crate::bout::{Bout, FencerScore, FencerVs, FencerVsError};
use crate::fencer::{Fencer, SimpleFencer};
use bout_creation::{BoutCreationError, BoutsCreator, SimpleBoutsCreator};

#[derive(Debug)]
pub enum PoolSheetError {
    Err1,
    Err2,
    Err3,
}

#[self_referencing]
#[derive(Debug)]
pub struct PoolSheet<T: Fencer + 'static> {
    fencers: Box<[T]>,
    #[borrows(fencers)]
    #[covariant]
    bouts: IndexMap<FencerVs<'this, T>, Bout<'this, T>, RandomState>,
}

impl<T: Fencer + 'static> PoolSheet<T> {
    pub fn create<C>(fencers: Vec<T>, creator: &C) -> Result<PoolSheet<T>, BoutCreationError>
    where
        C: BoutsCreator<T>,
    {
        Ok(PoolSheet::new(fencers.into_boxed_slice(), |fencers| {
            let mut bouts = IndexMap::new();
            match creator.get_order(fencers) {
                Ok(bout_indexes) => {
                    for pair in bout_indexes.into_iter() {
                        match FencerVs::new(
                            fencers.get(pair.0 - 1).unwrap(),
                            fencers.get(pair.1 - 1).unwrap(),
                        ) {
                            Ok(versus) => {
                                bouts.insert(versus.clone(), Bout::new(versus));
                            }
                            Err(err) => {
                                // return Err(BoutCreationError::VsError(
                                //     err,
                                //     "The pool creation paired a fencer with themselves."
                                //         .to_string(),
                                // ))
                                panic!("I should fix this");
                            }
                        }
                    }
                }
                Err(err) => {
                    // return Err(BoutCreationError::PoolOrderError(err));
                    panic!("I should fix this");
                }
            };
            bouts
        }))
    }
    // pub fn create(fencers: Vec<T>) -> Self {
    //     PoolSheet::new(fencers, |_| IndexMap::new())
    // }
    pub fn get_fencers(&self) -> &[T] {
        self.borrow_fencers()
    }

    pub fn get_bout<'a, 'b>(&'a mut self, versus: &FencerVs<'b, T>) -> Option<&Bout<'a, T>>
    where
        'b: 'a,
    {
        self.borrow_bouts().get(versus)
    }

    pub fn iter(&self) -> indexmap::map::Iter<FencerVs<T>, Bout<T>> {
        self.borrow_bouts().iter()
    }

    pub fn get_bouts(&self) -> &IndexMap<FencerVs<T>, Bout<T>> {
        self.borrow_bouts()
    }

    pub fn update_score(
        &mut self,
        fencer_a: FencerScore<T>,
        fencer_b: FencerScore<T>,
    ) -> Result<(), PoolSheetError>
// ) -> Result<&Bout<T>, PoolSheetError>
// where
        //     'b: 'a,
    {
        let x =
            FencerVs::new(fencer_a.fencer, fencer_b.fencer).map_err(|_| PoolSheetError::Err2)?;
        let (bout_key, _) = self.borrow_bouts().get_key_value(&x).unwrap();
        // let test_a = fencer_a.fencer.clone();
        // // let score_a = fencer_a.score;
        // let test_b = fencer_b.fencer.clone();
        // let score_b = fencer_b.score;

        // // let new_score_a = FencerScore::new(&test_a, fencer_a.score);
        // let new_score_b = FencerScore::new(&test_b, fencer_b.score);
        self.with_bouts_mut(|bouts| {
            let bout = bouts.get_mut(bout_key).ok_or(PoolSheetError::Err1)?;
            bout.update_score(&fencer_a, &fencer_b)
                .map_err(|_| PoolSheetError::Err3)?;

            Ok(())
        })?;
        // Err(PoolSheetError::Err1)
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use super::{PoolSheet, SimpleBoutsCreator};
    use crate::{
        bout::{FencerScore, FencerVs},
        cards::Cards,
        fencer::SimpleFencer,
    };

    #[test]
    fn basic_test() {
        let x = PoolSheet::new(vec![SimpleFencer::new("Hello")], |_| IndexMap::new());
    }
    // #[test]
    // fn iter_test() {
    //     let fencers = [
    //         SimpleFencer::new("Fencer1"),
    //         SimpleFencer::new("Fencer2"),
    //         SimpleFencer::new("Fencer3"),
    //         SimpleFencer::new("Fencer4"),
    //     ];

    //     let pool_sheet = PoolSheet::new(&fencers, &SimpleBoutsCreator).unwrap();
    //     for bout in pool_sheet.iter() {
    //         println!("{bout:#?}");
    //     }
    // }

    // #[test]
    // fn bout_addressing() {
    //     let mut fencers = [
    //         SimpleFencer::new("Fencer1"),
    //         SimpleFencer::new("Fencer2"),
    //         SimpleFencer::new("Fencer3"),
    //         SimpleFencer::new("Fencer4"),
    //     ];

    //     let json_fencer1 =
    //         serde_json::from_str::<SimpleFencer>(r#"{"name":"Fencer1","clubs":[]}"#).unwrap();
    //     let json_fencer2 =
    //         serde_json::from_str::<SimpleFencer>(r#"{"name":"Fencer2","clubs":[]}"#).unwrap();

    //     let mut pool_sheet = PoolSheet::new(&fencers, &SimpleBoutsCreator).unwrap();

    //     let a_versus = FencerVs::new(&json_fencer1, &json_fencer2).unwrap();

    //     pool_sheet
    //         .update_score(
    //             FencerScore {
    //                 fencer: &json_fencer1,
    //                 score: 0,
    //                 cards: Cards::default(),
    //             },
    //             FencerScore {
    //                 fencer: &json_fencer2,
    //                 score: 0,
    //                 cards: Cards::default(),
    //             },
    //         )
    //         .unwrap();
    //     println!("\nSingle Bout: {:#?}", pool_sheet.get_bout(&a_versus));
    // }
}

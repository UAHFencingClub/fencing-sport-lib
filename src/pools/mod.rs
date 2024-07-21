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
    fencers: Vec<T>,
    #[borrows(fencers)]
    #[covariant]
    bouts: IndexMap<FencerVs<'this, T>, Bout<'this, T>, RandomState>,
}

impl<T: Fencer> PoolSheet<T> {
    pub fn create(fencers: Vec<T>) -> Self {
        PoolSheet::new(fencers, |_| IndexMap::new())
    }
    pub fn get_fencers(&self) -> &Vec<T> {
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

    //     pub fn update_score(
    //         &mut self,
    //         fencer_a: FencerScore<T>,
    //         fencer_b: FencerScore<T>,
    //     ) -> Result<&Bout<T>, PoolSheetError>
    // // where
    //     //     'b: 'a,
    //     {
    //         let test_a = fencer_a.fencer.clone();
    //         // let score_a = fencer_a.score;
    //         let test_b = fencer_b.fencer.clone();
    //         // let score_b = fencer_b.score;

    //         // let new_score_a = FencerScore::new(&test_a, fencer_a.score);
    //         let new_score_b = FencerScore::new(&test_b, fencer_b.score);
    //         let x = self.with_bouts_mut(|bouts| {
    //             let x = FencerVs::new(&test_a, &test_b).map_err(|_| PoolSheetError::Err2)?;
    //             let bout = bouts.get_mut(&x).ok_or(PoolSheetError::Err1)?;
    //             let y = bout.update_score(&fencer_a, &fencer_b);
    //             // bout.update_score(&new_score_a, &new_score_b)
    //             // .map_err(|_| PoolSheetError::Err3)?;
    //             Ok(())
    //         })?;
    //         Err(PoolSheetError::Err1)
    //     }
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

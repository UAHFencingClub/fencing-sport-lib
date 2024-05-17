use std::collections::hash_map::RandomState;

use indexmap::IndexMap;

use crate::bout::{Bout, FencerScore, FencerVs, FencerVsError};
use crate::fencer::Fencer;
use crate::organizations::usafencing::pool_bout_orders::{get_default_order, PoolOrderError};

#[derive(Debug)]
pub enum BoutCreationError {
    VsError(FencerVsError, String),
    PoolOrderError(PoolOrderError),
}

pub trait BoutsCreator<T: Fencer> {
    fn get_order(&self, fencers: &[T]) -> Result<Vec<(usize, usize)>, PoolOrderError>;
}

pub struct SimpleBoutsCreator;

impl<T: Fencer> BoutsCreator<T> for SimpleBoutsCreator {
    fn get_order(&self, fencers: &[T]) -> Result<Vec<(usize, usize)>, PoolOrderError> {
        let fencer_count = fencers.len();
        get_default_order(fencer_count)
    }
}

#[derive(Debug)]
pub enum PoolSheetError {
    Err1,
    Err2,
    Err3,
}

#[derive(Debug)]
pub struct PoolSheet<'a, T: Fencer>(IndexMap<FencerVs<'a, T>, Bout<'a, T>, RandomState>);

impl<'a, T> PoolSheet<'a, T>
where
    T: Fencer,
{
    pub fn new<U: BoutsCreator<T>>(
        fencers: &'a mut [T],
        bout_creator: &U,
    ) -> Result<PoolSheet<'a, T>, BoutCreationError> {
        match bout_creator.get_order(fencers) {
            Ok(bout_indexes) => {
                let mut bouts_map = IndexMap::new();
                for pair in bout_indexes.into_iter() {
                    match FencerVs::new(
                        fencers.get(pair.0 - 1).unwrap(),
                        fencers.get(pair.1 - 1).unwrap(),
                    ) {
                        Ok(versus) => {
                            bouts_map.insert(versus.clone(), Bout::new(versus));
                        }
                        Err(err) => {
                            return Err(BoutCreationError::VsError(
                                err,
                                "The pool creation paired a fencer with themselves.".to_string(),
                            ))
                        }
                    }
                }
                Ok(PoolSheet(bouts_map))
            }
            Err(err) => Err(BoutCreationError::PoolOrderError(err)),
        }
    }

    pub fn iter(&self) -> indexmap::map::Iter<'_, FencerVs<'_, T>, Bout<'_, T>> {
        self.0.iter()
    }

    pub fn get_bout(&mut self, versus: &FencerVs<'a, T>) -> Option<&Bout<'a, T>> {
        self.0.get(versus)
    }

    pub fn update_score(
        &mut self,
        fencer_a: FencerScore<'a, T>,
        fencer_b: FencerScore<'a, T>,
    ) -> Result<&Bout<T>, PoolSheetError> {
        let versus =
            FencerVs::new(fencer_a.fencer, fencer_b.fencer).map_err(|_| PoolSheetError::Err2)?;
        let bout = self.0.get_mut(&versus).ok_or(PoolSheetError::Err1)?;
        bout.update_score(&fencer_a, &fencer_b)
            .map_err(|_| PoolSheetError::Err3)?;
        Ok(&*bout)
    }
    // pub fn get_bout_mut(&mut self, versus: &FencerVs<'a, T>) -> Option<&mut Bout<'a, T>>{
    //     self.0.get_mut(versus)
    // }

    pub fn get_results(&self) -> Result<(), PoolSheetError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{PoolSheet, SimpleBoutsCreator};
    use crate::{
        bout::{FencerScore, FencerVs},
        cards::Cards,
        fencer::SimpleFencer,
    };

    #[test]
    fn iter_test() {
        let mut fencers = [
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];

        let pool_sheet = PoolSheet::new(&mut fencers, &SimpleBoutsCreator).unwrap();
        for bout in pool_sheet.iter() {
            println!("{bout:#?}");
        }
    }

    #[test]
    fn bout_addressing() {
        let mut fencers = [
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];

        let json_fencer1 =
            serde_json::from_str::<SimpleFencer>(r#"{"name":"Fencer1","clubs":[]}"#).unwrap();
        let json_fencer2 =
            serde_json::from_str::<SimpleFencer>(r#"{"name":"Fencer2","clubs":[]}"#).unwrap();

        let mut pool_sheet = PoolSheet::new(&mut fencers, &SimpleBoutsCreator).unwrap();

        let a_versus = FencerVs::new(&json_fencer1, &json_fencer2).unwrap();

        pool_sheet
            .update_score(
                FencerScore {
                    fencer: &json_fencer1,
                    score: 0,
                    cards: Cards::default(),
                },
                FencerScore {
                    fencer: &json_fencer2,
                    score: 0,
                    cards: Cards::default(),
                },
            )
            .unwrap();
        println!("\nSingle Bout: {:#?}", pool_sheet.get_bout(&a_versus));
    }
}

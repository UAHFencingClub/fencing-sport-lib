use std::cell::RefCell;
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
pub struct PoolSheet<T: Fencer> {
    fencers: Vec<T>,
    pub bouts: IndexMap<FencerVs<T>, Bout<T>, RandomState>,
}

impl<T: Fencer> PoolSheet<T> {
    pub fn add_fencer(&mut self, fencer: T) {
        self.fencers.push(fencer);
    }

    pub fn add_fencers<I>(&mut self, fencers: I)
    where
        I: Iterator<Item = T>,
    {
        self.fencers.extend(fencers);
    }

    pub fn get_fencers(&self) -> &Vec<T> {
        &self.fencers
    }

    pub fn iter(&self) -> indexmap::map::Iter<FencerVs<T>, Bout<T>> {
        self.bouts.iter()
    }

    pub fn create_bouts<C>(&mut self, creator: &C) -> Result<(), BoutCreationError>
    where
        C: BoutsCreator<T>,
    {
        match creator.get_order(&self.fencers) {
            Ok(bout_indexes) => {
                for pair in bout_indexes.into_iter() {
                    match FencerVs::new(
                        self.fencers.get(pair.0 - 1).unwrap().clone(),
                        self.fencers.get(pair.1 - 1).unwrap().clone(),
                    ) {
                        Ok(versus) => {
                            self.bouts.insert(versus.clone(), Bout::new(versus));
                        }
                        Err(err) => {
                            return Err(BoutCreationError::VsError(
                                err,
                                "The pool creation paired a fencer with themselves.".to_string(),
                            ))
                        }
                    }
                }
                Ok(())
            }
            Err(err) => Err(BoutCreationError::PoolOrderError(err)),
        }
    }

    pub fn update_score(
        &mut self,
        fencer_a: FencerScore<T>,
        fencer_b: FencerScore<T>,
    ) -> Result<(), PoolOrderError> {
        let x = FencerVs::new(fencer_a.fencer.clone(), fencer_b.fencer.clone()).unwrap();
        let bout = self.bouts.get_mut(&x).unwrap();
        bout.update_score(fencer_a, fencer_b);
        Ok(())
    }
}

impl<T: Fencer> Default for PoolSheet<T> {
    fn default() -> Self {
        PoolSheet {
            fencers: Vec::new(),
            bouts: IndexMap::new(),
        }
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

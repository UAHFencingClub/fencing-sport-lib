use std::cell::RefCell;
use std::collections::hash_map::RandomState;

use indexmap::IndexMap;

use crate::bout::{Bout, FencerVs, FencerVsError};
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
pub struct PoolSheet<'a, T: Fencer> {
    fencers: Vec<T>,
    bouts: RefCell<IndexMap<FencerVs<'a, T>, Bout<'a, T>, RandomState>>,
}

impl<'a, 'b, T: Fencer> PoolSheet<'a, T> {
    pub fn add_fencer(&mut self, fencer: T) {
        self.fencers.push(fencer);
    }

    pub fn add_fencers<I>(&mut self, fencers: I)
    where
        I: Iterator<Item = T>,
    {
        self.fencers.extend(fencers);
    }

    pub fn get_fencers(&self) -> &Vec<T>{
        &self.fencers
    }

    pub fn create_bouts<C>(&'b self, creator: &C) -> Result<(), BoutCreationError>
    where
        C: BoutsCreator<T>,
        'b: 'a
    {
        match creator.get_order(&self.fencers) {
            Ok(bout_indexes) => {
                let mut bouts_map = self.bouts.borrow_mut();
                for pair in bout_indexes.into_iter() {
                    match FencerVs::new(
                        self.fencers.get(pair.0-1).unwrap(),
                        self.fencers.get(pair.1-1).unwrap()
                    ) {
                        Ok(versus) => {
                            bouts_map.insert(versus.clone(), Bout::new(versus));
                        },
                        Err(err) => {
                            return Err(BoutCreationError::VsError(err, "The pool creation paired a fencer with themselves.".to_string()))
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

impl<'a, T: Fencer> Default for PoolSheet<'a, T> {
    fn default() -> Self {
        PoolSheet {
            fencers: Vec::new(),
            bouts: RefCell::new(IndexMap::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bout::{FencerScore, FencerVs}, cards::Cards, fencer::SimpleFencer};
    use super::{PoolSheet, SimpleBoutsCreator};

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
        for bout in pool_sheet.bouts.borrow().iter() {
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

        let json_fencer1 = serde_json::from_str::<SimpleFencer>(r#"{"name":"Fencer1","clubs":[]}"#).unwrap();
        let json_fencer2 = serde_json::from_str::<SimpleFencer>(r#"{"name":"Fencer2","clubs":[]}"#).unwrap();

        let mut pool_sheet = PoolSheet::default();
        pool_sheet.add_fencers(fencers.clone().into_iter());
        let _ = pool_sheet.create_bouts(&SimpleBoutsCreator);

        let a_versus = FencerVs::new(&json_fencer1, &json_fencer2).unwrap();

        let mut bouts = pool_sheet.bouts.borrow_mut();
        let a_bout = bouts.get_mut(&a_versus).unwrap();
        a_bout.update_score(FencerScore {
            fencer: &json_fencer1,
            score: 0,
            cards: Cards::default(),
        },FencerScore {
            fencer: &json_fencer2,
            score: 0,
            cards: Cards::default(),
        });
        println!("\nSingle Bout: {a_bout:#?}");
    }
}
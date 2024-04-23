use std::collections::hash_map::RandomState;

use indexmap::IndexMap;

use crate::bout::{Bout, FencerVs, FencerVsError};
use crate::fencer::Fencer;
use crate::organizations::usafencing::pool_bout_orders::{get_default_order, PoolOrderError};

#[derive(Debug)]
pub enum PoolSheetError {
    NoBouts,
    VsError(FencerVsError, String),
    PoolOrderError(PoolOrderError),
}

pub trait BoutsCreator<T: Fencer> {
    fn get_order(&self, fencers: &[T]) -> Result<Vec<(usize, usize)>, PoolOrderError>;
}

struct SimpleBoutsCreator;

impl<T: Fencer> BoutsCreator<T> for SimpleBoutsCreator {
    fn get_order(&self, fencers: &[T]) -> Result<Vec<(usize, usize)>, PoolOrderError> {
        let fencer_count = fencers.len();
        get_default_order(fencer_count)
    }
}

#[derive(Debug)]
struct PoolSheet<'a, T: Fencer> {
    fencers: Vec<T>,
    bouts: IndexMap<FencerVs<'a, T>, Bout<'a, T>, RandomState>,
}

impl<'a, 'b, T: Fencer> PoolSheet<'a, T> {
    pub fn builder() -> PoolSheetBuilder<'a, T> {
        PoolSheetBuilder::default()
    }
}

pub struct PoolSheetBuilder<'a, T: Fencer> {
    fencers: Vec<T>,
    bouts: Result<IndexMap<FencerVs<'a, T>, Bout<'a, T>, RandomState>, PoolSheetError>,
}

impl<'a, T: Fencer> Default for PoolSheetBuilder<'a, T> {
    fn default() -> Self {
        PoolSheetBuilder {
            fencers: Vec::new(),
            bouts: Err(PoolSheetError::NoBouts),
        }
    }
}

impl <'a, T: Fencer> PoolSheetBuilder<'a, T> {
    fn add_fencer(mut self, fencer: T) -> Self {
        self.fencers.push(fencer);
        self
    }

    pub fn add_fencers<I>(mut self, fencers: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        self.fencers.extend(fencers);
        self
    }

    pub fn create_bouts<C>(mut self, creator: &C) -> Self
    where
        C: BoutsCreator<T>,
    {
        let test = &self.fencers;
        let bouts = match creator.get_order(test) {
            Ok(bout_indexes) => {
                let mut bouts = IndexMap::new();
                let mut status = Err(PoolSheetError::NoBouts);
                for pair in bout_indexes.into_iter() {
                    status = match FencerVs::<'a>::new(
                        self.fencers.get(pair.0-1).unwrap(),
                        self.fencers.get(pair.1-1).unwrap()
                    ) {
                        Ok(versus) => {
                            bouts.insert(versus.clone(), Bout::new(versus));
                            Ok(())
                        },
                        Err(err) => {
                            Err(PoolSheetError::VsError(err, "The pool creation paired a fencer with themselves.".to_string()))
                        }
                    };
                }
                match status {
                    Ok(()) => Ok(bouts),
                    Err(err) => Err(err)
                }
            }
            Err(err) => {
                Err(PoolSheetError::PoolOrderError(err))
            }
        };
        self.bouts = bouts;
        self
    }
    
    pub fn build(self) -> Result<PoolSheet<'a, T>, PoolSheetError>{
        match self.bouts {
            Ok(bouts) => Ok(PoolSheet {
                fencers: self.fencers,
                bouts
            }),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bout::{self, FencerVs}, fencer::SimpleFencer};
    use super::{BoutsCreator, PoolSheet, SimpleBoutsCreator};

    #[test]
    fn iter_test() {
        let fencers = [
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];

        let pool_sheet = PoolSheet::builder()
            .add_fencers(fencers.into_iter())
            .create_bouts(&SimpleBoutsCreator)
            .build().unwrap();
        for bout in pool_sheet.bouts {
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

        let mut pool_sheet = PoolSheet::builder()
            .add_fencers(fencers.clone().into_iter())
            .create_bouts(&SimpleBoutsCreator)
            .build().unwrap();

        let a_versus = FencerVs::new(&fencers[0], &fencers[1]).unwrap();

        let a_bout = pool_sheet.bouts.get_mut(&a_versus).unwrap();
        a_bout.update_score(0,5);
        println!("\nSingle Bout: {a_bout:#?}");

    }
}
use std::collections::hash_map::RandomState;

use indexmap::IndexMap;

use crate::bout::{Bout, FencerVs, FencerVsError};
use crate::fencer::{self, Fencer};
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

struct FencerScore<T> {
    fencer: T,
    score: usize,
}
pub struct FencersScores<T: Fencer>(FencerScore<T>, FencerScore<T>);

#[derive(Debug)]
pub struct PoolSheet<T: Fencer> {
    fencers: Vec<T>,
    bouts: IndexMap<FencerVs<T>, Bout<T>, RandomState>,
}

impl<T: Fencer> PoolSheet<T> {
    pub fn builder() -> PoolSheetBuilder<T> {
        PoolSheetBuilder::default()
    }

    pub fn update_scores(&mut self, score: FencersScores<T>) -> Result<(),PoolSheetError> {
        let fencer_vs = match FencerVs::new(score.0.fencer, score.1.fencer){
            Ok(versus) => versus,
            Err(err) => {
                return Err(PoolSheetError::VsError(err, "ToDo".to_string()));
            }
        };
        let bout = match self.bouts.get_mut(&fencer_vs) {
            Some(bout) => bout,
            None => {
                return Err(PoolSheetError::NoBouts);
            }
        };
        bout
        Ok(())
    }
}

pub struct PoolSheetBuilder<T: Fencer> {
    fencers: Vec<T>,
    bouts: Result<IndexMap<FencerVs<T>, Bout<T>, RandomState>, PoolSheetError>,
}

impl<T: Fencer> Default for PoolSheetBuilder<T> {
    fn default() -> Self {
        PoolSheetBuilder {
            fencers: Vec::new(),
            bouts: Err(PoolSheetError::NoBouts),
        }
    }
}

impl <T: Fencer> PoolSheetBuilder<T> {
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
                    status = match FencerVs::new(
                        self.fencers.get(pair.0-1).unwrap().clone(),
                        self.fencers.get(pair.1-1).unwrap().clone()
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
    
    pub fn build(self) -> Result<PoolSheet<T>, PoolSheetError>{
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
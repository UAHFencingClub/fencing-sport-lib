use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::default;
use std::io::SeekFrom;

use indexmap::map::Iter;
use indexmap::IndexMap;

use crate::bout::{Bout, FencerVs, FencerVsError};
use crate::fencer::{self, Fencer};
use crate::organizations::usafencing::pool_bout_orders::{get_default_order, PoolOrderError};

// #[derive(Debug)]
// pub enum BoutCreationError {
//     VsError(FencerVsError, String),
//     PoolOrderError(PoolOrderError),
// }

pub trait BoutsCreator<T: Fencer> {
    fn get_order(&self, fencers: &mut [T]) -> Result<Vec<(usize, usize)>, PoolOrderError>;
}

pub struct SimpleBoutsCreator;

impl<T: Fencer> BoutsCreator<T> for SimpleBoutsCreator {
    fn get_order(&self, fencers: &mut [T]) -> Result<Vec<(usize, usize)>, PoolOrderError> {
        let fencer_count = fencers.len();
        get_default_order(fencer_count)
    }
}

#[derive(Debug)]
pub enum PoolSheetError {
    PoolNotFinished,
    BoutOrderUnspecified,
    BoutOrderError(PoolOrderError),
}

impl From<PoolOrderError> for PoolSheetError {
    fn from(value: PoolOrderError) -> Self {
        PoolSheetError::BoutOrderError(value)
    }
}

pub struct PoolSheet<'a, T: Fencer> {
    fencers: Vec<T>,
    bout_order: Vec<(usize, usize)>,
    bouts: HashMap<FencerVs<'a, T>, Bout<'a, T>>
}

impl<'a, T> PoolSheet<'a, T> 
where
    T: Fencer,
{
    pub fn builder<U: BoutsCreator<T>>() -> PoolSheetBuilder<'a, T, U> {
        PoolSheetBuilder::default()
    }

    pub fn get_bout(&mut self, versus: FencerVs<'a, T>) -> &Bout<'a, T>{
        // TODO examine the use of clone here
        self.bouts.entry(versus.clone()).or_insert_with(|| {
            assert!(self.fencers.contains(versus.0) && self.fencers.contains(versus.1));
            Bout::new(versus)
        })
    }

    pub fn get_bout_mut(&mut self, versus: FencerVs<'a, T>) -> &mut Bout<'a, T>{
        self.bouts.entry(versus.clone()).or_insert_with(|| {
            assert!(self.fencers.contains(versus.0) && self.fencers.contains(versus.1));
            Bout::new(versus)
        })
    }

    pub fn get_results(&self) -> Result<(), PoolSheetError> {
        todo!()
    }
}

impl <'a, 'b, T: Fencer> Iterator for PoolSheet<'a, T>
{
    type Item = &'a Bout<'a, T>;
    fn next(&mut self) -> Option<Self::Item>
    {
        todo!()
        // let (fencer_a_index, fencer_b_index) = self.bout_order.iter().next()?;
        // let fencer_a = self.fencers.get(*fencer_a_index).unwrap();
        // let fencer_b = self.fencers.get(*fencer_b_index).unwrap();
        // let versus = FencerVs::new(fencer_a, fencer_b).unwrap();
        // Some(self.get_bout(versus))
    }
}

pub struct PoolSheetBuilder<'a, T: Fencer, U: BoutsCreator<T>> {
    fencers: Vec<T>,
    bout_order: Option<U>,
    bouts: HashMap<FencerVs<'a, T>, Bout<'a, T>>
}

impl <'a, T: Fencer, U: BoutsCreator<T>> Default for PoolSheetBuilder<'a, T, U> {
    fn default() -> Self {
        PoolSheetBuilder {
            fencers: Vec::new(),
            bout_order: None,
            bouts: HashMap::new(),
        }
    }
}

impl <'a, T: Fencer, U: BoutsCreator<T>> PoolSheetBuilder<'a, T, U> {
    pub fn add_fencers<I>(mut self, fencers: I) -> Self
    where
        I: Iterator<Item = T>
    {
        self.fencers.extend(fencers);
        self
    }

    pub fn add_fencer(mut self, fencer: T) -> Self {
        self.fencers.push(fencer);
        self
    }

    pub fn with_bout_order(mut self, bout_creator: U) -> Self {
        self.bout_order = Some(bout_creator);
        self
    }

    pub fn build(mut self) -> Result<PoolSheet<'a, T>, PoolSheetError> {
        let bout_order = self.bout_order.ok_or(PoolSheetError::BoutOrderUnspecified)?.get_order(&mut self.fencers)?;
        Ok(PoolSheet {
            fencers: self.fencers,
            bout_order,
            bouts: HashMap::new()
        })
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

        let pool_sheet = PoolSheet::builder()
            .add_fencers(fencers.into_iter())
            .with_bout_order(SimpleBoutsCreator)
            .build()
            .unwrap();

        for bout in pool_sheet {
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

        let mut pool_sheet = PoolSheet::builder()
            .add_fencers(fencers.into_iter())
            .with_bout_order(SimpleBoutsCreator)
            .build()
            .unwrap();

        let a_versus = FencerVs::new(&json_fencer1, &json_fencer2).unwrap();

        let a_bout = pool_sheet.get_bout_mut(a_versus);
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
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::default;
use std::io::SeekFrom;
use std::ops::Index;
use std::slice::SliceIndex;
use std::hash;
use std::hash::Hash;

use indexmap::map::Iter;
use indexmap::IndexMap;

use crate::bout::{Bout, FencerVsError};
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

// Maybe make this take in boxes to fencers?
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone, Copy)]
pub struct PoolVs<T: Fencer>(T, T);

impl<T: Fencer> PoolVs<T>{
    pub fn new(fencer_a: T, fencer_b: T) -> Result<Self, FencerVsError>{
        if fencer_a == fencer_b {
            return Err(FencerVsError::SameFencer);
        }
        Ok(PoolVs(fencer_a,fencer_b))
    }
}

impl<T: Fencer> Hash for PoolVs<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match self {
            PoolVs(a, b) if a > b => {
                a.hash(state);
                b.hash(state);
            }
            PoolVs(a, b) if b > a => {
                b.hash(state);
                a.hash(state);
            }
            _ => {panic!("A FencerVs struct should not have its 2 items be the same.")}
        }
    }
}

struct PoolFencerScore {
    fencer: usize,
    score: usize,
}
pub struct PoolBout{
    fencers: (usize, usize),
    scores: Option<(PoolFencerScore, PoolFencerScore)>,
}

pub struct PoolSheet<T: Fencer> {
    fencers: Vec<T>,
    bout_order: Vec<(usize, usize)>,
    bouts: HashMap<(usize, usize), PoolBout>
}

impl<T> PoolSheet<T> 
where
    T: Fencer,
{
    pub fn builder<U: BoutsCreator<T>>() -> PoolSheetBuilder<T, U> {
        PoolSheetBuilder::default()
    }

    pub fn get_bout(&mut self, versus: PoolVs<T>) -> &Bout<T>{
        // TODO examine the use of clone here
        self.bouts.entry(versus.clone()).or_insert_with(|| {
            assert!(self.fencers.0.contains(versus.0) && self.fencers.0.contains(versus.1));
            Bout::new(versus)
        })
    }

    pub fn get_bout_mut(&mut self, versus: PoolVs<T>) -> &mut Bout<T>{
        self.bouts.entry(versus.clone()).or_insert_with(|| {
            assert!(self.fencers.0.contains(versus.0) && self.fencers.0.contains(versus.1));
            Bout::new(versus)
        })
    }

    pub fn get_results(&self) -> Result<(), PoolSheetError> {
        todo!()
    }
}

impl <T: Fencer> Iterator for PoolSheet<T>
{
    type Item = (&'a T, &'a T);
    fn next(&mut self) -> Option<Self::Item>
    {
        // todo!()
        let (fencer_a_index, fencer_b_index) = self.bout_order.iter().next()?;
        let fencer_a = self.fencers.0.get(*fencer_a_index).unwrap();
        let fencer_b = self.fencers.0.get(*fencer_b_index).unwrap();
        Some((fencer_a, fencer_b))
    }
}

pub struct PoolSheetBuilder<T: Fencer, U: BoutsCreator<T>> {
    fencers: Vec<T>,
    bout_order: Option<U>,
}

impl <T: Fencer, U: BoutsCreator<T>> Default for PoolSheetBuilder<T, U> {
    fn default() -> Self {
        PoolSheetBuilder {
            fencers: Vec::new(),
            bout_order: None,
        }
    }
}

impl <T: Fencer, U: BoutsCreator<T>> PoolSheetBuilder<T, U> {
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

    pub fn build(mut self) -> Result<PoolSheet<T>, PoolSheetError> {
        let bout_order = self.bout_order.ok_or(PoolSheetError::BoutOrderUnspecified)?.get_order(&mut self.fencers)?;

        Ok(PoolSheet {
            fencers: self.fencers,
            bout_order,
            bouts: HashMap::new()
        })
    }
}

struct FencerList<T>(Vec<T>);

impl<T> FencerList<T> {
    fn new(fencers: Vec<T>) -> Self {
        FencerList(fencers)
    }
}

impl<T, Idx> Index<Idx> for FencerList<T>
where
    Idx: SliceIndex<[T], Output = T>,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Idx) -> &Self::Output {
        self.0.index(index)
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

        println!("Address of initial fencers: {:p}", &fencers[0]);
        println!("Address of initial fencer0: {:p}", fencers.get(0).unwrap());


        let mut pool_sheet = PoolSheet::builder()
            .add_fencers(fencers.into_iter())
            .with_bout_order(SimpleBoutsCreator)
            .build()
            .unwrap();

        println!("Address of poolsheet fencers: {:p}", &pool_sheet.fencers);
        for i in 0..4 {
        println!("Address of individual fencer[{i}]: {:p}", pool_sheet.fencers.0.get(i).unwrap());
        }

        println!("Json Fencers: {:p} {:p}",&json_fencer1,&json_fencer2);
        let a_versus = FencerVs::new(&json_fencer1, &json_fencer2).unwrap();

        let a_bout = pool_sheet.get_bout_mut(a_versus);
        println!("The fencer in bout {:p}", a_bout.fencers.0);
        println!("The fencer in bout {:p}", a_bout.fencers.1);
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
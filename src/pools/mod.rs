use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use indexmap::map::Iter;
use indexmap::{IndexMap, IndexSet};
pub use result::PoolResults;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeStruct, SerializeTupleStruct};
use serde::{Deserialize, Serialize, Serializer};
use serializer_structs::{PoolSheetSpecialBoutsList, PoolSheetSpecialFencers};

use crate::bout::{Bout, FencerScore, FencerVs};
use crate::fencer::Fencer;
use bout_creation::BoutsCreator;

pub mod bout_creation;
mod pool_error;
pub use pool_error::PoolSheetError;
mod deserializer_struct;
pub mod result;
mod serializer_structs;

pub type PoolSheetFencerScore<T> = FencerScore<T, Rc<T>>;
pub type PoolSheetVersus<T> = FencerVs<T, Rc<T>>;
pub type PoolSheetBout<T> = Bout<T, Rc<T>>;
pub type PoolBoutIter<'a, T> = Iter<'a, FencerVs<T, Rc<T>>, Bout<T, Rc<T>>>;
type PoolSheetMap<T> = IndexMap<PoolSheetVersus<T>, PoolSheetBout<T>, RandomState>;

#[derive(Debug, Clone)]
pub struct PoolSheet<T: Fencer> {
    fencers: Box<[Rc<T>]>,
    bouts: IndexMap<PoolSheetVersus<T>, PoolSheetBout<T>, RandomState>,
}

impl<T: Fencer> PoolSheet<T> {
    pub fn new<C>(fencers: IndexSet<T>, creator: &C) -> Result<PoolSheet<T>, PoolSheetError>
    where
        C: BoutsCreator<T>,
    {
        let fencers: Vec<T> = fencers.into_iter().collect();
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

    pub fn get_bout_mut<U: Borrow<T> + Clone + Eq>(
        &mut self,
        vs: &FencerVs<T, U>,
    ) -> Result<&mut PoolSheetBout<T>, PoolSheetError> {
        let a = Rc::new(vs.0.borrow().clone());
        let b = Rc::new(vs.1.borrow().clone());
        let vs = FencerVs::new(a, b).unwrap();
        self.bouts.get_mut(&vs).ok_or(PoolSheetError::NoBoutFound)
    }

    pub fn iter_bouts(&self) -> indexmap::map::Iter<FencerVs<T, Rc<T>>, Bout<T, Rc<T>>> {
        self.bouts.iter()
    }

    pub fn update_score<U: Borrow<T> + Clone + Eq>(
        &mut self,
        fencer_a: FencerScore<T, U>,
        fencer_b: FencerScore<T, U>,
    ) -> Result<(), PoolSheetError> {
        // Need to convert fencerscore struct since the index map needs a version using an Rc smart pointer.
        // This does mean calling this function requires 2 heap allocations every time it is used
        // I did do some earlier testing in previous commits to make sure that the data is dropped after this function call
        let fencer_a_fencer = Rc::new(fencer_a.fencer.borrow().clone());
        let fencer_b_fencer = Rc::new(fencer_b.fencer.borrow().clone());

        let x = FencerVs::new(fencer_a_fencer, fencer_b_fencer)?;
        let (_, vs, bout) = self
            .bouts
            .get_full_mut(&x)
            .ok_or(PoolSheetError::NoBoutFound)?;

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

        bout.update_scores(fencer_a, fencer_b)
    }

    pub fn unset_score<U: Borrow<T> + Clone + Eq>(
        &mut self,
        fencer_a: FencerScore<T, U>,
        fencer_b: FencerScore<T, U>,
    ) -> Result<(), PoolSheetError> {
        // Need to convert fencerscore struct since the index map needs a version using an Rc smart pointer.
        // This does mean calling this function requires 2 heap allocations every time it is used
        // I did do some earlier testing in previous commits to make sure that the data is dropped after this function call
        let fencer_a_fencer = Rc::new(fencer_a.fencer.borrow().clone());
        let fencer_b_fencer = Rc::new(fencer_b.fencer.borrow().clone());

        let x = FencerVs::new(fencer_a_fencer, fencer_b_fencer)?;
        let (_, _, bout) = self
            .bouts
            .get_full_mut(&x)
            .ok_or(PoolSheetError::NoBoutFound)?;

        bout.unset_scores();
        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        for (_, bout) in self.bouts.iter() {
            if bout.get_winner().is_none() {
                return false;
            }
        }
        true
    }

    pub fn finish(&self) -> Result<PoolResults<T>, PoolSheetError> {
        if self.is_finished() {
            Ok(PoolResults::new(self))
        } else {
            Err(PoolSheetError::PoolNotComplete)
        }
    }
}

impl<T: Fencer + Serialize> Serialize for PoolSheet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PoolSheet", 2)?;
        state.serialize_field(
            "fencers",
            &PoolSheetSpecialFencers::from(self.fencers.clone()),
        )?;
        state.serialize_field(
            "bouts",
            &PoolSheetSpecialBoutsList::from(self.bouts.clone()),
        )?;
        state.end()
    }
}

#[derive(Debug)]
struct DeserPoolSheet<T: Fencer> {
    fencers: deserializer_struct::Fencers<T>,
    bouts: deserializer_struct::Bouts,
}

// Implement Deserialize for DeserPoolSheet
impl<'de, T> Deserialize<'de> for DeserPoolSheet<T>
where
    T: Fencer + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Fencers,
            Bouts,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`fencers` or `bouts`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "fencers" => Ok(Field::Fencers),
                            "bouts" => Ok(Field::Bouts),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct MyStructVisitor<T: Fencer> {
            marker: PhantomData<fn() -> DeserPoolSheet<T>>,
        }

        impl<'de, T> Visitor<'de> for MyStructVisitor<T>
        where
            T: Fencer + Deserialize<'de>,
        {
            type Value = DeserPoolSheet<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct DeserPoolSheet")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<DeserPoolSheet<T>, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let fencers = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let bouts = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(DeserPoolSheet { fencers, bouts })
            }

            fn visit_map<A>(self, mut map: A) -> Result<DeserPoolSheet<T>, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut fencers = None;
                let mut bouts = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Fencers => {
                            if fencers.is_some() {
                                return Err(de::Error::duplicate_field("fencers"));
                            }
                            fencers = Some(map.next_value()?);
                        }
                        Field::Bouts => {
                            if bouts.is_some() {
                                return Err(de::Error::duplicate_field("bouts"));
                            }
                            bouts = Some(map.next_value()?);
                        }
                    }
                }
                let fencers = fencers.ok_or_else(|| de::Error::missing_field("fencers"))?;
                let bouts = bouts.ok_or_else(|| de::Error::missing_field("bouts"))?;
                Ok(DeserPoolSheet { fencers, bouts })
            }
        }

        const FIELDS: &[&str] = &["fencers", "bouts"];
        deserializer.deserialize_struct(
            "DeserPoolSheet",
            FIELDS,
            MyStructVisitor {
                marker: PhantomData,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexSet;

    use super::{bout_creation::SimpleBoutsCreator, DeserPoolSheet, PoolSheet};
    use crate::{bout::FencerScore, cards::Cards, fencer::SimpleFencer};

    #[test]
    fn from_vec_test() {
        let fencers = vec![
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];

        let _pool_sheet =
            PoolSheet::new(IndexSet::from_iter(fencers), &SimpleBoutsCreator).unwrap();
    }

    #[test]
    fn iter_test() {
        let fencers = [
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];

        let pool_sheet = PoolSheet::new(fencers.into(), &SimpleBoutsCreator).unwrap();
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

        let mut pool_sheet = PoolSheet::new(fencers.into(), &SimpleBoutsCreator).unwrap();

        let _smth = pool_sheet.update_score(
            FencerScore::new(json_fencer1, 0, Cards::default()),
            FencerScore::new(json_fencer2, 0, Cards::default()),
        );
        println!("\nSingle Bout: {pool_sheet:#?}");
    }

    #[test]
    #[ignore = "Requires manual inspection"]
    fn serialize_poolsheet() {
        let fencers = [
            SimpleFencer::new("Fencer1"),
            SimpleFencer::new("Fencer2"),
            SimpleFencer::new("Fencer3"),
            SimpleFencer::new("Fencer4"),
        ];
        let mut pool_sheet = PoolSheet::new(fencers.clone().into(), &SimpleBoutsCreator).unwrap();
        pool_sheet
            .update_score(
                FencerScore::new(fencers[0].clone(), 3, Cards::default()),
                FencerScore::new(fencers[1].clone(), 5, Cards::default()),
            )
            .unwrap();
        let json_out = serde_json::to_string_pretty(&pool_sheet).unwrap();

        println!("Json: {json_out}");
    }

    #[test]
    fn update_score_unordered() {
        // Make sure that the order of inputting scores does not matter.
        todo!();
    }

    #[test]
    fn deser_poolsheet() {
        let input = r#"
            {
                "fencers": {
                    "140300542545664": {
                        "name": "Fencer1",
                        "clubs": []
                    },
                    "140300542545744": {
                        "name": "Fencer2",
                        "clubs": []
                    }
                },
                "bouts": {
                    "[140300542545664,140300542545744]": {
                        "scores": [
                            3,
                            5
                        ],
                        "cards": [
                            {
                                "yellow": 0,
                                "red": 0,
                                "group3red": 0,
                                "black": 0,
                                "passivity_yellow": 0,
                                "passivity_red": 0,
                                "passivity_black": 0
                            },
                            {
                                "yellow": 0,
                                "red": 0,
                                "group3red": 0,
                                "black": 0,
                                "passivity_yellow": 0,
                                "passivity_red": 0,
                                "passivity_black": 0
                            }
                        ],
                        "priority": "None"
                    }
                }
            }
        "#;

        let test: DeserPoolSheet<SimpleFencer> = serde_json::from_str(input).unwrap();

        println!("{test:?}")
    }
}

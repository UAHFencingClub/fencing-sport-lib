use std::{borrow::Borrow, hash::RandomState, rc::Rc};

use indexmap::IndexMap;
use serde::{
    ser::{Error, SerializeMap, SerializeStruct, SerializeTupleStruct},
    Serialize, Serializer,
};

use super::{PoolSheetBout, PoolSheetMap, PoolSheetVersus};

use crate::fencer::Fencer;

pub(crate) struct PoolSheetSpecialVs<T: Fencer>(PoolSheetVersus<T>);
pub(crate) struct PoolSheetSpecialBout<T: Fencer>(PoolSheetBout<T>);
pub(crate) struct PoolSheetSpecialFencers<T: Fencer>(Box<[Rc<T>]>);
pub(crate) struct PoolSheetSpecialBoutsList<T: Fencer>(
    IndexMap<PoolSheetVersus<T>, PoolSheetBout<T>, RandomState>,
);

impl<T: Fencer> From<PoolSheetVersus<T>> for PoolSheetSpecialVs<T> {
    fn from(value: PoolSheetVersus<T>) -> Self {
        PoolSheetSpecialVs(value)
    }
}

impl<T: Fencer + Serialize> Serialize for PoolSheetSpecialVs<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let vs = &self.0;
        let id1 = Rc::as_ptr(&vs.0) as usize;
        let id2 = Rc::as_ptr(&vs.1) as usize;
        let mut state = serializer.serialize_tuple_struct("FencerVs", 2)?;
        state.serialize_field(&id1)?;
        state.serialize_field(&id2)?;
        state.end()
    }
}

impl<T: Fencer + Serialize> Serialize for PoolSheetSpecialBout<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PoolBout", 3)?;
        state.serialize_field("scores", &self.0.scores)?;
        state.serialize_field("cards", &self.0.cards)?;
        state.serialize_field("priority", &self.0.priority)?;
        state.end()
    }
}

impl<T: Fencer + Serialize> From<PoolSheetBout<T>> for PoolSheetSpecialBout<T> {
    fn from(value: PoolSheetBout<T>) -> Self {
        PoolSheetSpecialBout(value)
    }
}

impl<T: Fencer + Serialize> Serialize for PoolSheetSpecialFencers<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fencers: &[Rc<T>] = self.0.borrow();
        let mut map = serializer.serialize_map(Some(fencers.len()))?;
        for fencer in fencers {
            let pointer = Rc::as_ptr(fencer) as usize;
            map.serialize_entry::<str, T>(&pointer.to_string(), fencer.borrow())?;
        }
        map.end()
    }
}

impl<T: Fencer + Serialize> From<Box<[Rc<T>]>> for PoolSheetSpecialFencers<T> {
    fn from(value: Box<[Rc<T>]>) -> Self {
        PoolSheetSpecialFencers(value)
    }
}

// struct SpecialError;
// impl Error for SpecialError {
//     fn custom<T>(msg:T) -> Self where T:Display {

//     }
// }

impl<T: Fencer + Serialize> Serialize for PoolSheetSpecialBoutsList<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bout_list = &self.0;
        let mut map = serializer.serialize_map(Some(bout_list.len()))?;
        for (key, val) in bout_list {
            let new_key = serde_json::to_string(&PoolSheetSpecialVs::from(key.clone()))
                .map_err(|err| Error::custom(format!("Error Creating the BoutList Key {err:?}")))?;
            let new_val = PoolSheetSpecialBout::from(val.clone());
            map.serialize_entry(&new_key, &new_val)?;
        }
        map.end()
    }
}

impl<T: Fencer + Serialize> From<PoolSheetMap<T>> for PoolSheetSpecialBoutsList<T> {
    fn from(value: PoolSheetMap<T>) -> Self {
        PoolSheetSpecialBoutsList(value)
    }
}

use std::{fmt, hash::RandomState, marker::PhantomData, rc::Rc};

use indexmap::IndexMap;
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::fencer::Fencer;

#[derive(Debug, PartialEq)]
struct Fencers<T: Fencer> {
    fencers: IndexMap<usize, Rc<T>, RandomState>,
}

impl<T: Fencer> Fencers<T> {
    fn with_capacity(capacity: usize) -> Self {
        Fencers {
            fencers: IndexMap::with_capacity(capacity),
        }
    }

    fn insert(&mut self, key: usize, value: Rc<T>) {
        self.fencers.insert(key, value);
    }
}

impl<'de, T> Deserialize<'de> for Fencers<T>
where
    T: Fencer + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FencersVisitor<T>
        where
            T: Fencer,
        {
            marker: PhantomData<fn() -> Fencers<T>>,
        }

        impl<T> FencersVisitor<T>
        where
            T: Fencer,
        {
            fn new() -> Self {
                FencersVisitor {
                    marker: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for FencersVisitor<T>
        where
            T: Fencer + Deserialize<'de>,
        {
            type Value = Fencers<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a custom map")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = Fencers::with_capacity(access.size_hint().unwrap_or(0));
                while let Some((key, value)) = access.next_entry()? {
                    map.insert(key, Rc::new(value));
                }
                Ok(map)
            }
        }

        deserializer.deserialize_map(FencersVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::fencer::SimpleFencer;

    use super::Fencers;

    #[test]
    fn deserialize_fencermap() {
        let input = r#"{
                "140300542545744": {
                    "name": "Fencer1",
                    "clubs": []
                },
                "140300542545664": {
                    "name": "Fencer2",
                    "clubs": []
                }
            }"#;

        let fencer1 = SimpleFencer::new("Fencer1");
        let fencer2 = SimpleFencer::new("Fencer2");

        let mut map = Fencers::with_capacity(2);
        map.insert(140300542545744, Rc::new(fencer1));
        map.insert(140300542545664, Rc::new(fencer2));

        let test: Fencers<SimpleFencer> = serde_json::from_str(input).unwrap();

        assert_eq!(map, test)
    }
}

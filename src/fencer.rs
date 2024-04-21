use serde::{Serialize, Deserialize};
use std::{cmp::Ordering, hash::Hash};

pub trait Fencer {
    fn dummy1(&self);
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[derive(Hash)]
pub struct SimpleFencer {
    name: String,
    clubs: Vec<Club>
}

impl Fencer for &SimpleFencer {
    fn dummy1(&self) {
        
    }
}

impl Fencer for SimpleFencer {
    fn dummy1(&self) {}
}

impl SimpleFencer{
    pub fn new(name: impl ToString) -> Self {
        SimpleFencer {
            name: name.to_string(),
            clubs: Vec::new()
        }
    }
}

impl PartialEq for SimpleFencer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for SimpleFencer {}

impl PartialOrd for SimpleFencer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SimpleFencer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Club {
    full_name: String,
    shortname: String,
}
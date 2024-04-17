use serde::{Serialize, Deserialize};
use time::Date;
use std::{cmp::Ordering, hash::{Hash, Hasher}};

#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Name {
    first_name: String,
    last_name: String,
    nick_name: Option<String>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
enum Hand {
    Left,
    Right,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Fencer {
    name: Name,
    clubs: Vec<Club>,
    date_of_birth: Option<Date>,
    gender: Option<Gender>,
    handedness: Option<Hand>,
}

impl Hash for Fencer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.date_of_birth.hash(state);
    }
}

impl PartialEq for Fencer {
    fn eq(&self, other: &Self) -> bool {
        (self.name == other.name) & (self.date_of_birth == other.date_of_birth)
    }
}
impl Eq for Fencer {}

impl PartialOrd for Fencer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fencer {
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
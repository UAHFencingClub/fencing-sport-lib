use crate::fencer::Fencer;
use crate::organizations::usafencing::club::Club;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
};
use time::Date;

#[derive(Debug, Hash, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Suffix {
    Todo1,
    Todo2,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone)]
struct Name {
    suffix: Option<Suffix>,
    first_name: String,
    last_name: String,
    middle_initial: Option<char>,
    nickname: Option<String>,
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let middle_initial = match self.middle_initial {
            Some(middle_initial) => middle_initial.to_string(),
            None => "".to_string(),
        };
        let nickname = match &self.nickname {
            Some(nickname) => format!("({nickname})"),
            None => "".to_string(),
        };
        write!(
            f,
            "{}, {} {} {}",
            self.last_name, self.first_name, nickname, middle_initial
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum GenderIdentity {
    Man,
    Woman,
    NonConforming,
    MaleToFemale,
    FemaleToMale,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Hand {
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct USAFFencer {
    name: Name,
    clubs: Vec<Club>,
    date_of_birth: Option<Date>,
    gender_identity: Option<GenderIdentity>,
    handedness: Option<Hand>,
}

impl Fencer for USAFFencer {
    fn get_fullname(&self) -> String {
        self.name.to_string()
    }
}
// Temporary
impl USAFFencer {
    pub fn new(name: String) -> Self {
        USAFFencer {
            name: Name {
                suffix: None,
                first_name: name,
                last_name: String::from(""),
                middle_initial: None,
                nickname: None,
            },
            clubs: Vec::new(),
            date_of_birth: None,
            gender_identity: None,
            handedness: None,
        }
    }
}

impl Hash for USAFFencer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.date_of_birth.hash(state);
    }
}

impl PartialEq for USAFFencer {
    fn eq(&self, other: &Self) -> bool {
        (self.name == other.name) & (self.date_of_birth == other.date_of_birth)
    }
}
impl Eq for USAFFencer {}

impl PartialOrd for USAFFencer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for USAFFencer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl USAFFencer {
    pub fn with_name(name: String) -> Self {
        USAFFencer {
            name: Name {
                suffix: None,
                first_name: name,
                last_name: "".to_string(),
                middle_initial: None,
                nickname: None,
            },
            clubs: Vec::new(),
            date_of_birth: None,
            gender_identity: None,
            handedness: None,
        }
    }
}

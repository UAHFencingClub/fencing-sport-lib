use serde::{Serialize, Deserialize};
use time::Date;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
#[derive(Hash)]
#[derive(Serialize, Deserialize)]
struct Name {
    first_name: String,
    last_name: String,
    nick_name: Option<String>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
enum Hand {
    Left,
    Right,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Fencer {
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

#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
struct Club {
    full_name: String,
    shortname: String,
}
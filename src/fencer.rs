use serde::{Serialize, Deserialize};
use std::{cmp::Ordering, hash::Hash};

pub trait Fencer: Hash + Serialize + Eq + PartialEq + PartialOrd + Ord + Clone{
    fn get_fullname(&self) -> String;
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct SimpleFencer {
    name: String,
    clubs: Vec<Club>
}

impl Fencer for SimpleFencer {
    fn get_fullname(&self) -> String {
        self.name.clone()
    }
}

impl SimpleFencer{
    pub fn new(name: impl ToString) -> Self {
        SimpleFencer {
            name: name.to_string(),
            clubs: Vec::new()
        }
    }
}

impl Hash for SimpleFencer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
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

#[cfg(test)]
mod tests {
    use super::SimpleFencer;
    use serde_json;

    #[test]
    fn serialize_test() {
        let fencer = SimpleFencer::new("Fencer1");
        let serialized_fencer = serde_json::to_string(&fencer).unwrap();
        println!("Serialized Fencer: {}", serialized_fencer.clone());
        let deser_fencer = serde_json::from_str::<SimpleFencer>(&serialized_fencer).unwrap();
        println!("Serialized Fencer: {:?}", deser_fencer);
    }
}
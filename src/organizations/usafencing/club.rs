use serde::{Serialize, Deserialize};
use crate::organizations::usafencing::{USState, ContactInfo, ClubRegion, Division};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Club {
    name: String,
    shortname: String,
    id: usize,
    //not sure what this date is for
    date: time::Date,
    // Maybe some binding to libpostal,
    // maybe some other bining
    //String for now.
    address: String,
    state: USState,
    point_of_contact: ContactInfo,
    region: ClubRegion,
    division: Division,
}
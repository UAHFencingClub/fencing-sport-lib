use crate::organizations::usafencing::{ClubRegion, ContactInfo, Division, USState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

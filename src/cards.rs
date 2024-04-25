use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct Cards {
    yellow: u8,
    red: u8,
    group3red: u8,
    black: u8,
    passivity_yellow: u8,
    passivity_red: u8,
    passivity_black: u8,
}
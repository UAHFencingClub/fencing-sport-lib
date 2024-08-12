use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
pub struct Cards {
    yellow: u8,
    red: u8,
    group3red: u8,
    black: u8,
    passivity_yellow: u8,
    passivity_red: u8,
    passivity_black: u8,
}

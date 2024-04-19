// Generative AI used to format the lists from
// https://cdn1.sportngin.com/attachments/document/0034/5494/bout_order.pdf
// These orders need to be validated

// Regular Pool Orders
pub const POOL_OF_4_ORDER: [(usize, usize); 6] = [
    (1, 4), (2, 3), (1, 3), (2, 4), (3, 4), (1, 2)
];
pub const POOL_OF_5_ORDER: [(usize, usize); 10] = [
    (1, 2), (3, 4), (5, 1), (2, 3), (5, 4), (1, 3), (2, 5), (4, 1), (3, 5), (4, 2)
];
pub const POOL_OF_6_ORDER: [(usize, usize); 15] = [
    (1, 2), (4, 5), (2, 3), (5, 6), (3, 1), (6, 4), (2, 5), (1, 4), (5, 3), (1, 6),
    (4, 2), (3, 6), (5, 1), (3, 4), (6, 2)
];
pub const POOL_OF_7_ORDER: [(usize, usize); 21] = [
    (1, 4), (2, 5), (3, 6), (7, 1), (5, 4), (2, 3), (6, 7), (5, 1), (4, 3), (6, 2),
    (5, 7), (3, 1), (4, 6), (7, 2), (3, 5), (1, 6), (2, 4), (7, 3), (6, 5), (1, 2),
    (4, 7)
];

// Special Bout Orders
pub const POOL_OF_6_SPECIAL_ORDER_1: [(usize, usize); 15] = [
    (1, 4), (2, 5), (3, 6), (5, 1), (4, 2), (3, 1), (6, 2), (5, 3), (6, 4), (1, 2),
    (3, 4), (5, 6), (2, 1), (4, 5), (6, 3)
];
pub const POOL_OF_6_SPECIAL_ORDER_2: [(usize, usize); 15] = [
    (1, 6), (4, 5), (2, 1), (5, 3), (3, 4), (6, 1), (5, 2), (4, 6), (1, 2), (6, 2),
    (4, 3), (5, 1), (3, 6), (2, 4), (3, 5)
];
pub const POOL_OF_7_SPECIAL_ORDER_1: [(usize, usize); 21] = [
    (1, 4), (2, 5), (3, 6), (7, 1), (5, 4), (2, 3), (6, 7), (5, 1), (4, 3), (6, 2),
    (5, 7), (3, 1), (4, 6), (7, 2), (3, 5), (1, 6), (2, 4), (7, 3), (6, 5), (1, 2),
    (4, 7)
];
pub const POOL_OF_7_SPECIAL_ORDER_2: [(usize, usize); 21] = [
    (1, 6), (2, 7), (3, 5), (4, 1), (5, 7), (6, 2), (7, 3), (5, 4), (6, 1), (7, 5),
    (1, 2), (3, 6), (4, 5), (7, 4), (5, 2), (6, 3), (7, 1), (2, 4), (5, 3), (1, 4),
    (6, 4)
];
pub const POOL_OF_8_SPECIAL_ORDER_1: [(usize, usize); 28] = [
    (2, 3), (1, 5), (7, 4), (6, 8), (1, 2), (3, 4), (5, 6), (8, 7), (4, 1), (5, 2),
    (8, 3), (6, 7), (4, 2), (8, 1), (7, 5), (3, 6), (2, 8), (5, 4), (6, 1), (3, 7),
    (4, 8), (2, 6), (3, 5), (1, 7), (4, 6), (8, 5), (7, 2), (1, 3)
];
pub const POOL_OF_8_SPECIAL_ORDER_2: [(usize, usize); 28] = [
    (3, 1), (6, 2), (4, 5), (8, 6), (3, 2), (7, 1), (5, 3), (4, 6), (8, 7), (5, 1),
    (6, 4), (2, 7), (1, 4), (6, 3), (8, 2), (5, 7), (3, 8), (6, 1), (7, 5), (4, 3),
    (2, 8), (5, 4), (7, 2), (1, 8), (3, 4), (6, 5), (7, 3), (8, 4)
];
pub const POOL_OF_9_SPECIAL_ORDER_1: [(usize, usize); 36] = [
    (1, 9), (2, 8), (3, 7), (4, 6), (1, 5), (2, 9), (8, 3), (7, 4), (6, 5), (1, 2),
    (9, 3), (8, 4), (7, 5), (6, 1), (3, 2), (9, 4), (5, 8), (7, 6), (3, 1), (2, 4),
    (5, 9), (8, 6), (7, 1), (4, 3), (5, 2), (6, 9), (8, 7), (4, 1), (5, 3), (6, 2),
    (9, 7), (1, 8), (4, 5), (3, 6), (2, 7), (9, 8)
];
pub const POOL_OF_9_SPECIAL_ORDER_2: [(usize, usize); 36] = [
    (2, 3), (6, 7), (4, 5), (3, 1), (8, 9), (1, 2), (7, 3), (5, 8), (6, 2), (4, 9),
    (7, 1), (3, 6), (9, 5), (8, 4), (2, 7), (1, 6), (9, 3), (8, 2), (5, 1), (4, 7),
    (6, 9), (3, 8), (2, 5), (1, 4), (9, 7), (8, 6), (5, 3), (4, 2), (9, 1), (7, 8),
    (5, 6), (3, 4), (2, 9), (1, 8), (7, 5), (6, 4)
];

// Regular Pool Orders
pub const POOL_OF_10_ORDER: [(usize, usize); 45] = [
    (1, 4), (6, 9), (2, 5), (7, 10), (3, 1), (8, 6), (4, 5), (9, 10), (2, 3), (7, 8),
    (5, 1), (10, 6), (4, 2), (9, 7), (5, 3), (10, 8), (1, 2), (6, 7), (3, 4), (8, 9),
    (5, 10), (1, 6), (2, 7), (3, 8), (4, 9), (6, 5), (10, 2), (8, 1), (7, 4), (9, 3),
    (2, 6), (5, 8), (4, 10), (1, 9), (3, 7), (8, 2), (6, 4), (9, 5), (10, 3), (7, 1),
    (4, 8), (2, 9), (3, 6), (5, 7), (1, 10)
];
pub const POOL_OF_11_ORDER: [(usize, usize); 55] = [
    (1, 2), (7, 8), (4, 5), (10, 11), (2, 3), (8, 9), (5, 6), (3, 1), (9, 7), (6, 4),
    (2, 5), (8, 11), (1, 4), (7, 10), (5, 3), (11, 9), (1, 6), (4, 2), (10, 8), (3, 6),
    (5, 1), (11, 7), (3, 4), (9, 10), (6, 2), (1, 7), (3, 9), (10, 4), (8, 2), (5, 11),
    (1, 8), (9, 2), (3, 10), (4, 11), (6, 7), (9, 1), (2, 10), (11, 3), (7, 5), (6, 8),
    (10, 1), (11, 2), (4, 7), (8, 5), (6, 9), (11, 1), (7, 3), (4, 8), (9, 5), (6, 10),
    (2, 7), (8, 3), (4, 9), (10, 5), (6, 11)
];

pub const POOL_OF_12_ORDER: [(usize, usize); 66] = [
    (1, 2), (7, 8), (4, 5), (10, 11), (2, 3), (8, 9), (5, 6), (11, 12), (3, 1), (9, 7),
    (6, 4), (12, 10), (2, 5), (8, 11), (1, 4), (7, 10), (5, 3), (11, 9), (1, 6), (7, 12),
    (4, 2), (10, 8), (3, 6), (9, 12), (5, 1), (11, 7), (3, 4), (9, 10), (6, 2), (12, 8),
    (1, 7), (3, 9), (10, 4), (8, 2), (5, 11), (12, 6), (1, 8), (9, 2), (3, 10), (4, 11),
    (12, 5), (6, 7), (9, 1), (2, 10), (11, 3), (4, 12), (7, 5), (6, 8), (10, 1), (11, 2),
    (12, 3), (4, 7), (8, 5), (6, 9), (11, 1), (2, 12), (7, 3), (4, 8), (9, 5), (6, 10),
    (12, 1), (2, 7), (8, 3), (4, 9), (10, 5), (6, 11)
];

#[derive(Debug)]
pub enum PoolOrderError {
    UnsupportedParticipantCount(String),
}

// Maybe make a macro to generate this and rename a few things.
pub fn get_default_order(num_fencers: usize) -> Result<Vec<(usize, usize)>, PoolOrderError>{
    match num_fencers {
        4 => Ok(POOL_OF_4_ORDER.to_vec()),
        5 => Ok(POOL_OF_5_ORDER.to_vec()),
        6 => Ok(POOL_OF_6_ORDER.to_vec()),
        7 => Ok(POOL_OF_7_ORDER.to_vec()),
        8 => Ok(POOL_OF_8_SPECIAL_ORDER_1.to_vec()),
        9 => Ok(POOL_OF_9_SPECIAL_ORDER_1.to_vec()),
        10 => Ok(POOL_OF_10_ORDER.to_vec()),
        11 => Ok(POOL_OF_11_ORDER.to_vec()),
        12 => Ok(POOL_OF_12_ORDER.to_vec()),
        _ => Err(PoolOrderError::UnsupportedParticipantCount("Can not create a bout order for the amount of fencers passed in".to_string()))
    }
}

// Associating special bout orders with their corresponding pool size
pub const SPECIAL_ORDERS: [(&[(usize, usize)], usize); 8] = [
    (&POOL_OF_6_SPECIAL_ORDER_1, 6),
    (&POOL_OF_6_SPECIAL_ORDER_2, 6),
    (&POOL_OF_7_SPECIAL_ORDER_1, 7),
    (&POOL_OF_7_SPECIAL_ORDER_2, 7),
    (&POOL_OF_8_SPECIAL_ORDER_1, 8),
    (&POOL_OF_8_SPECIAL_ORDER_2, 8),
    (&POOL_OF_9_SPECIAL_ORDER_1, 9),
    (&POOL_OF_9_SPECIAL_ORDER_2, 9)
];

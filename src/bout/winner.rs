use std::{borrow::Borrow, marker::PhantomData};

use crate::fencer::Fencer;

pub enum BoutWinner<T, U>
where
    T: Fencer,
    U: Borrow<T>,
{
    Auto(PhantomData<T>),
    Manual(U),
}

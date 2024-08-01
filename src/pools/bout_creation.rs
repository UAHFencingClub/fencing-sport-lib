use crate::fencer::Fencer;
use crate::organizations::usafencing::pool_bout_orders::get_default_order;
use crate::pools::PoolSheetError;

pub trait BoutsCreator<T: Fencer> {
    fn get_order(&self, fencers: &[T]) -> Result<Vec<(usize, usize)>, PoolSheetError>;
}

pub struct SimpleBoutsCreator;

impl<T: Fencer> BoutsCreator<T> for SimpleBoutsCreator {
    fn get_order(&self, fencers: &[T]) -> Result<Vec<(usize, usize)>, PoolSheetError> {
        let fencer_count = fencers.len();
        get_default_order(fencer_count)
    }
}

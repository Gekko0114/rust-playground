use crate::money::{Money, ReduceTrait};
use crate::bank::Bank;
pub struct Sum {
    pub augend: Money,
    pub addend: Money
}
impl ReduceTrait for Sum {
    fn reduce(&self, _bank: &Bank, to: &'static str ) -> Money {
        Money {
            amount: &self.augend.amount + &self.addend.amount,
            currency: to            
        }
    }
}
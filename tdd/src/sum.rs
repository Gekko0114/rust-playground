use crate::money::{Money, ReduceTrait};
use crate::bank::Bank;
pub struct Sum {
    pub augend: Money,
    pub addend: Money
}
impl ReduceTrait for Sum {
    fn reduce(&self, bank: &Bank, to: &'static str ) -> Money {
        Money {
            amount: &self.augend.reduce(bank, to).amount + &self.addend.reduce(bank, to).amount,
            currency: to            
        }
    }
}
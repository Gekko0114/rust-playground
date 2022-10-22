use crate::money::{Money, ReduceTrait};
pub struct Sum {
    pub augend: Money,
    pub addend: Money
}
impl ReduceTrait for Sum {
    fn reduce(&self, to: &'static str ) -> Money {
        Money {
            amount: &self.augend.amount + &self.addend.amount,
            currency: to            
        }
    }
}
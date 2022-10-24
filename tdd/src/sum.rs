use crate::money::{Money, ExpressionTrait};
use crate::bank::Bank;
pub struct Sum {
    pub augend: Box<dyn ExpressionTrait>,
    pub addend: Box<dyn ExpressionTrait>
}
impl ExpressionTrait for Sum {
    fn reduce(&self, bank: &Bank, to: &'static str ) -> Money {
        Money {
            amount: &self.augend.reduce(bank, to).amount + &self.addend.reduce(bank, to).amount,
            currency: to            
        }
    }
}
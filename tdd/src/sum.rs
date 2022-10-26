use crate::money::{Money, ExpressionTrait, PlusTrait};
use crate::bank::Bank;

pub struct Sum {
    pub augend: Box<dyn ExpressionTrait + 'static>,
    pub addend: Box<dyn ExpressionTrait + 'static>
}
impl ExpressionTrait for Sum {
    fn reduce(&self, bank: &Bank, to: &'static str ) -> Money {
        Money {
            amount: &self.augend.reduce(bank, to).amount + &self.addend.reduce(bank, to).amount,
            currency: to            
        }
    }
}

impl PlusTrait for Sum {
    fn plus<T: ExpressionTrait + 'static>(self, object: T) -> Sum {
        Sum {
            augend: Box::new(Sum {
                augend: self.augend,
                addend: self.addend
            }),
            addend: Box::new(object)
        }
    }
}
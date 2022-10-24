use crate::sum::Sum;
use crate::bank::Bank;

#[derive(Eq, PartialEq, Debug)]
pub struct Money {
    pub amount: i32,
    pub currency: &'static str
}

impl Money {    
    pub fn times(&self, multiplier: i32) -> Money {
        return Money {amount: self.amount * multiplier, currency: self.currency};
    }

    pub fn equals(&self, object: Money) -> bool {        
        (self.amount == object.amount) && (self.currency == object.currency)
    }

    pub fn dollar(amount: i32) -> Money {
        Money {amount: amount, currency: "USD"}
    }

    pub fn franc(amount: i32) -> Money {
        Money {amount: amount, currency: "CHF"}
    }

    pub fn currency(&self) -> &'static str {
        self.currency
    }

    pub fn plus(&self, object: &Money) -> Sum {
        Sum {
            augend: Box::new(Money {
                amount: self.amount,
                currency: self.currency,
            }),
            addend: Box::new(Money {
                amount: object.amount,
                currency: object.currency,
            })
        }
    }
}

pub struct Expression {
    pub amount: i32,
    pub currency: &'static str
}

pub trait ExpressionTrait {
    fn reduce(&self, bank: &Bank, to: &'static str) -> Money;
}

impl ExpressionTrait for Money {
    fn reduce(&self, bank: &Bank, to: &'static str ) -> Money {
        let rate = bank.rate(self.currency, to);
        Money {
            amount: self.amount / rate,
            currency: to            
        }
    }
}
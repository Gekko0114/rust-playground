#[derive(Eq, PartialEq, Debug)]
pub struct Money {
    pub amount: i32
}

impl Money {    
    pub fn times(&self, multiplier: i32) -> Money {
        return Money {amount: self.amount * multiplier};
    }

    pub fn equals(&self, object: Money) -> bool {
        self.amount == object.amount
    }

    pub fn dollar(amount: i32) -> Money {
        Money {amount: amount}
    }

    pub fn franc(amount: i32) -> Money {
        Money {amount: amount}
    }
}

pub trait MoneyTrait {
    fn new(amount: i32) -> Money;
}
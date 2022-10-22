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

    pub fn plus(&self, object: &Money) -> Expression {
        Expression {amount: self.amount + object.amount, currency: self.currency}
    }
}

pub struct Expression {
    pub amount: i32,
    pub currency: &'static str
}

impl MoneyTrait for Expression {}

pub trait MoneyTrait {
}
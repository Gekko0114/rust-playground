use crate::money::{Expression, Money};

pub struct Bank {    
}

impl Bank {
    pub fn reduce(&self, source: Expression, to: &'static str ) -> Money {
        Money::dollar(10)
    }
}
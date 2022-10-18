use crate::money::{Money, MoneyTrait};

#[derive(Eq, PartialEq, Debug)]
pub struct Franc {
    amount: i32
}

impl MoneyTrait for Franc {
    fn new(amount: i32) -> Money {
        Money {
            amount: amount,
        }
    }
}
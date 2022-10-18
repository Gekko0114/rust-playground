use crate::money::{Money, MoneyTrait};

#[derive(Eq, PartialEq, Debug)]
pub struct Dollar {
    amount: i32
}

impl MoneyTrait for Dollar {
    fn new(amount: i32) -> Money {
        Money {
            amount: amount,
        }
    }
}
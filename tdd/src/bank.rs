use crate::money::{Money, ExpressionTrait};
use std::collections::HashMap;

#[derive(Default)]
pub struct Bank {
    pub rates: HashMap<String, i32>
}

impl Bank {
    pub fn reduce<R: ExpressionTrait>(&self, source: R, to: &'static str ) -> Money {
        source.reduce(self, to)
    }

    pub fn add_rate(&mut self, from: &'static str, to: &'static str, rate: i32) {
        let key = String::from(from) + to;
        self.rates.insert(key, rate);
    }

    pub fn rate(&self, from: &'static str, to: &'static str) -> i32 {
        if from == to {
            1
        } else {
            let key = String::from(from) + to;
            match self.rates.get(&key) {
                Some(i) => *i,
                _ => 1
            }
        }
}
}
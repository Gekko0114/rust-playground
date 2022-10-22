use crate::money::{Money, ReduceTrait};

pub struct Bank {    
}

impl Bank {
    pub fn reduce<R: ReduceTrait>(&self, source: R, to: &'static str ) -> Money {
        source.reduce(to)
    }
}
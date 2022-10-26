pub mod money;
pub mod bank;
pub mod sum;

#[cfg(test)]
mod tests {
#[test]
pub fn test_multiplication() {
    use super::*;
    let five = money::Money::dollar(5);
    assert!(money::Money::dollar(10).equals(five.times(2)));
}
#[test]
pub fn test_equality() {
    use super::*;
    assert!(money::Money::dollar(5).equals(money::Money::dollar(5)));
    assert!(!money::Money::dollar(5).equals(money::Money::dollar(1)));
    assert!(!money::Money::dollar(5).equals(money::Money::franc(5)));
}

#[test]
pub fn test_currency() {
    use super::*;
    assert_eq!(money::Money::dollar(1).currency(), "USD");
    assert_eq!(money::Money::franc(1).currency(), "CHF");
}

#[test]
pub fn test_simple_addition() {
    use super::*;
    use crate::money::PlusTrait;

    let five = money::Money::dollar(5);
    let sum = five.plus(five.clone());
    let bank: bank::Bank = Default::default();
    let reduced = bank.reduce(sum, "USD");
    assert!(money::Money::dollar(10).equals(reduced));
}

#[test]
pub fn test_reduce_sum() {
    use super::*;
    let sum = sum::Sum {augend: Box::new(money::Money::dollar(3)), addend: Box::new(money::Money::dollar(4))};
    let bank: bank::Bank = Default::default();
    let result = bank.reduce(sum, "USD");
    assert!(result.equals(money::Money::dollar(7)));
}

#[test]
pub fn test_reduce_money() {
    use super::*;
    let bank: bank::Bank = Default::default();
    let result = bank.reduce(money::Money::dollar(1), "USD");
    assert!(result.equals(money::Money::dollar(1)));
}

#[test]
pub fn test_reduce_money_different_currency() {
    use super::*;
    let mut bank: bank::Bank = Default::default();
    bank.add_rate("CHF", "USD", 2);
    let result = bank.reduce(money::Money::franc(2), "USD");
    assert!(money::Money::dollar(1).equals(result));
}

#[test]
pub fn test_identity_rate() {
    use super::*;
    let bank: bank::Bank = Default::default();
    assert_eq!(1, bank.rate("USD", "USD"));
}

#[test]
pub fn test_mixed_addition() {
    use super::*;
    use crate::money::PlusTrait;
    
    let five_dollar = money::Money::dollar(5);
    let ten_franc = money::Money::franc(10);
    let mut bank: bank::Bank = Default::default();
    bank.add_rate("CHF", "USD", 2);
    let result = bank.reduce(five_dollar.plus(ten_franc), "USD");
    assert!(money::Money::dollar(10).equals(result));
}

#[test]
pub fn test_sum_plus_money() {
    use super::*;
    use crate::money::PlusTrait;

    let five_dollar = money::Money::dollar(5);
    let ten_franc = money::Money::franc(10);
    let mut bank: bank::Bank = Default::default();
    bank.add_rate("CHF", "USD", 2);
    let sum = sum::Sum{augend: Box::new(five_dollar), addend: Box::new(ten_franc)}.plus(five_dollar);
    let result = bank.reduce(sum, "USD");
    assert!(money::Money::dollar(15).equals(result));
}
}
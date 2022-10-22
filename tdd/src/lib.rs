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
    let five = money::Money::dollar(5);
    let sum = five.plus(&five);
    let bank = bank::Bank{};
    let reduced = bank.reduce(sum, "USD");
    assert!(money::Money::dollar(10).equals(reduced));
}

#[test]
pub fn test_plus_returns_sum() {
    use super::*;
    let five = money::Money::dollar(5);
    let sum = five.plus(&five);
    assert!(five.equals(sum.augend));
    assert!(five.equals(sum.addend));
}

#[test]
pub fn test_reduce_sum() {
    use super::*;
    let sum = sum::Sum {augend: money::Money::dollar(3), addend: money::Money::dollar(4)};
    let bank = bank::Bank{};
    let result = bank.reduce(sum, "USD");
    assert!(result.equals(money::Money::dollar(7)));
}

#[test]
pub fn test_reduce_money() {
    use super::*;
    let bank = bank::Bank{};
    let result = bank.reduce(money::Money::dollar(1), "USD");
    assert!(result.equals(money::Money::dollar(1)));
}
}
pub mod dollar;
pub mod franc;
pub mod money;
use money::MoneyTrait;

#[cfg(test)]
mod tests {
#[test]
pub fn test_multiplication() {
    use super::*;
    let five = money::Money::dollar(5);
    assert!(money::Money::dollar(10).equals(five.times(2)));
    assert!(money::Money::dollar(15).equals(five.times(3)));
}
#[test]
pub fn test_equality() {
    use super::*;
    assert!(money::Money::dollar(5).equals(money::Money::dollar(5)));
    assert!(money::Money::dollar(5).equals(money::Money::franc(5)));
}

#[test]
pub fn test_flanc_multiplication() {
    use super::*;
    let five = money::Money::franc(5);
    assert!(money::Money::franc(10).equals(five.times(2)));
    assert!(money::Money::franc(15).equals(five.times(3)));
}
}
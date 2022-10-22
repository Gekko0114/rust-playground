pub mod money;

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
    assert!(!money::Money::dollar(5).equals(money::Money::franc(5)));
}

#[test]
pub fn test_flanc_multiplication() {
    use super::*;
    let five = money::Money::franc(5);
    assert!(money::Money::franc(10).equals(five.times(2)));
    assert!(money::Money::franc(15).equals(five.times(3)));
}

#[test]
pub fn test_currency() {
    use super::*;
    assert_eq!(money::Money::dollar(1).currency(), "USD");
    assert_eq!(money::Money::franc(1).currency(), "CHF");
}
}
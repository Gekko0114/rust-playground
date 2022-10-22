pub mod money;

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
}
pub mod dollar;
pub mod franc;
pub mod money;
use money::MoneyTrait;

#[cfg(test)]
mod tests {
#[test]
pub fn test_multiplication() {
    use super::*;
    let five = dollar::Dollar::new(5);
    assert!(dollar::Dollar::new(10).equals(five.times(2)));
    assert!(dollar::Dollar::new(15).equals(five.times(3)));
}
#[test]
pub fn test_equality() {
    use super::*;
    assert!(dollar::Dollar::new(5).equals(dollar::Dollar::new(5)));
}

#[test]
pub fn test_flanc_multiplication() {
    use super::*;
    let five = franc::Franc::new(5);
    assert!(franc::Franc::new(10).equals(five.times(2)));
    assert!(franc::Franc::new(15).equals(five.times(3)));
}
}
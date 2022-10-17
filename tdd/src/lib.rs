mod dollar;

#[cfg(test)]
mod tests {
#[test]
pub fn test_money() {
    use super::*;
    let dollar = dollar::Dollar::new(5);
    assert_eq!(dollar.times(2), 10);
}
}
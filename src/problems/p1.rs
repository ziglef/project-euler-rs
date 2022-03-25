pub fn p1(limit: u32) -> u32 {
    let mut sum = 0;
    for n in 0..limit {
        if n % 3 == 0 || n % 5 == 0 {
            sum = sum + n;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(10), 23);
    }
}
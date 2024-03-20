use plus_one66::plus_one;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        let digits: Vec<i32> = [1, 2, 3].to_vec();
        let result: Vec<i32> = plus_one(digits);
        let target: Vec<i32> = [1, 2, 4].to_vec();
        assert_eq!(result, target)
    }
}

use my_sqrt69::my_sqrt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        let num: i32 = 8;
        let result: i32 = my_sqrt(num);
        assert_eq!(result, 2)
    }
}

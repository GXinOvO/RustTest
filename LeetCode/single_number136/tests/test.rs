use single_number136::single_number;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        let nums: Vec<i32> = [4, 1, 2, 1, 2].to_vec();
        let result: i32 = single_number(nums);
        assert_eq!(result, 4)
    }
}

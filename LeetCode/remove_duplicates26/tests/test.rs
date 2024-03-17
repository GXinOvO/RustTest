use remove_duplicates26::remove_duplicates;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums: Vec<i32> = [0, 0, 1, 1, 2, 2, 3, 3, 4].to_vec();
        let result: i32 = remove_duplicates(&mut nums);
        assert_eq!(result, 5);
    }
}

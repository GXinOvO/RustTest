use remove_element27::remove_element;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums: Vec<i32> = [0, 1, 2, 2, 3, 0, 4, 2].to_vec();
        let result: i32 = remove_element(&mut nums, 2);
        assert_eq!(result, 5);
    }
}

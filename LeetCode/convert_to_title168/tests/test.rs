use convert_to_title168::convert_to_title;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_title() {
        let mut nums: i32 = 701;
        let target: String = "ZY".to_string();
        let result: String = convert_to_title(nums);
        assert_eq!(result, target)
    }
}

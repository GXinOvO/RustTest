use isValid20::is_valid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let s: String = "()[]{}".to_string();
        let result: bool = is_valid(s);
        assert_eq!(result, true);
    }
}

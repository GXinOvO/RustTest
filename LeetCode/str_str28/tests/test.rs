use str_str28::str_str;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        let haystack: String = "sadbutsad".to_string();
        let needle: String = "sad".to_string();
        let result: i32 = str_str(haystack, needle);
        assert_eq!(result, 0)
    }
}

use length_of_last_word58::length_of_last_word;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        let s: String = "Hello World".to_string();
        let result: i32 = length_of_last_word(s);
        assert_eq!(result, 5)
    }
}

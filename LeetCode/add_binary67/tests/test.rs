use add_binary67::add_binary;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        let a: String = "1010".to_string();
        let b: String = "1011".to_string();

        let result: String = add_binary(a, b);
        assert_eq!(result, "10101".to_string())
    }
}

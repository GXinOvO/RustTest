use title_to_number171::title_to_number;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_to_number() {
        let columnTitle: String = "ZY".to_string();
        let target: i32 = 701;
        let result: i32 = title_to_number(columnTitle);
        assert_eq!(target, result);
    }
}

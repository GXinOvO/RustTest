use is_isomorphic205::is_isomorphic;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        let s: String = "egg".to_string();
        let t: String = "add".to_string();
        let target: bool = true;
        let result = is_isomorphic(s, t);
        assert_eq!(target, result);
    }
}

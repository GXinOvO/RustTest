use super::is_palindroma;

#[test]
fn test_is_palindroma() {
    let s = "A man, a plan, a canal: Panama".to_string();
    assert_eq!(is_palindroma(s), true)
}
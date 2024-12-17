use super::reverse_words;

#[test]
fn test_reverse_words() {
    let s = String::from("the sky is blue");
    let result = String::from("blue is sky the");
    assert_eq!(reverse_words(s), result)
}
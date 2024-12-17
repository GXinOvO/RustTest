use super::candy;

#[test]
fn test_candy() {
    let ratings = vec![1, 0, 2];
    assert_eq!(candy(ratings), 5)
}
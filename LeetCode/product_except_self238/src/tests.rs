use super::product_except_self;

#[test]
fn test_product_except_self() {
    let pre = vec![1, 2, 3, 4];
    let result = vec![24, 12, 8, 6];

    assert_eq!(product_except_self(pre), result)
}
use super::{ jump };

#[test]
fn test_jump() {
    let values = vec![2, 3, 1, 1, 4];
    let result = 2;
    assert_eq!(jump(values), result)
}
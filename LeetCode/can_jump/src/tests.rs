use super::{ can_jump };

#[test]
fn test_can_jump() {
    let values = vec![2, 3, 1, 1, 4];
    let result = true;
    assert_eq!(can_jump(values), result)
}
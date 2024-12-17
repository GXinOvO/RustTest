use super::trap;

#[test]
fn test_trap() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(trap(height), 6)
}
use super::can_complete_circuit;

#[test]
fn test_can() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    assert_eq!(can_complete_circuit(gas, cost), 3)

}
use super::RandomizedSet;

#[test]
fn test() {
    let obj = RandomizedSet::new();
    let ret_1: bool = obj.insert(1);
    assert_eq!(ret_1, true);
    let ret_2: bool = obj.remove(2);
    assert_eq!(ret_2, false);
    let ret_3: bool = obj.insert(2);
    assert_eq!(ret_3, true);
    let ret_4: i32 = obj.get_random();
    let ret_5: bool = obj.remove(1);
    assert_eq!(ret_5, true);
    let ret_6: bool = obj.insert(2);
    assert_eq!(ret_6, false);
    let ret_7: i32 = obj.get_random();
    assert_eq!(ret_7, 2);

}



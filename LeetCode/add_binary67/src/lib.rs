use std::cmp;

pub fn add_binary(a: String, b: String) -> String {
    let mut vec_a: Vec<i32> = a.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let mut vec_b: Vec<i32> = b.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let len = cmp::max(vec_a.len(), vec_b.len());

    let mut r_vec: Vec<i32> = Vec::new();

    let mut carry: i32 = 0;

    for _i in 0..len {
        let l_a = vec_a.pop().unwrap_or(0);
        let l_b = vec_b.pop().unwrap_or(0);
        let mut r_s = l_a + l_b + carry;
        carry = 0;
        if r_s >= 2 {
            r_s -= 2;
            carry = 1;
        }
        r_vec.insert(0, r_s);
    }
    if carry == 1 {
        r_vec.insert(0, 1);
    }

    let joined: String = r_vec.iter().map(ToString::to_string).collect();
    return joined;
}

pub fn my_sqrt(x: i32) -> i32 {
    let (mut left, mut right) = (0 as i64, x as i64);

    let mut res = 0;
    let xx: i64 = x as i64;

    while left <= right {
        let mid = (left + (right - left) / 2) as i64;
        if mid * mid <= xx {
            res = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    res as i32
}

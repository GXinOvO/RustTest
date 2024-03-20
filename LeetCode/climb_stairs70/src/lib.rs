pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }

    let mut prev_prev = 1;
    let mut prev = 2;
    for _i in 3..n + 1 {
        let next = prev_prev + prev;
        prev_prev = prev;
        prev = next;
    }
    prev
}

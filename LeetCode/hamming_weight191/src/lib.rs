pub fn hamming_weight(n: u32) -> i32 {
    let (mut ret, mut mask) = (0, 1u32);

    for _i in 0..32 {
        if n & mask != 0 {
            ret += 1;
        }
        mask = mask << 1;
    }
    ret
}

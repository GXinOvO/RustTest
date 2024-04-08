pub fn reverse_bits(x: u32) -> u32 {
    let mut bits = vec![];
    let mut x = x;
    while x > 0 {
        bits.push(x & 1);
        x = x >> 1;
    }
    while bits.len() < 32 {
        bits.push(0);
    }

    bits.iter().fold(0, |acc, &b| acc * 2 + b)
}

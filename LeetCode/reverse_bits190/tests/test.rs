use reverse_bits190::reverse_bits;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        let n = "00000010100101000001111010011100";
        let n = u32::from_str_radix(n, 2).unwrap();
        let target: u32 = 964176192;
        let result: u32 = reverse_bits(n);
        assert_eq!(target, result)
    }
}

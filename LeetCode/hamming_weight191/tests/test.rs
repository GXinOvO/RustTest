use hamming_weight191::hamming_weight;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        let n = 2147483645;
        let target = 30;
        let result = hamming_weight(n);
        assert_eq!(target, result);
    }
}

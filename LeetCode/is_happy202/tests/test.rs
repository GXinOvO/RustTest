use is_happy202::is_happy;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        let n = 19;
        let target = true;
        let result = is_happy(n);
        assert_eq!(target, result);
    }
}

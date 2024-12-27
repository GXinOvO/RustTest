pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() { return true; }

    let s = s.as_bytes();
    let mut i = 0;
    for c in t.bytes() {
        if s[i] == c {
            i += 1;
            if i == s.len() {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        let s = "abc".to_string();
        let t = String::from("ahbgdc");
        assert_eq!(is_subsequence(s, t), true);
    }
}

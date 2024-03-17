pub fn str_str(haystack: String, needle: String) -> i32 {
    let l = haystack.len();
    let n = needle.len();

    let hb = haystack.as_bytes();
    let nb = needle.as_bytes();

    if n < 1 {
        return 0;
    }
    if l < n {
        return -1;
    }

    for i in 0..=l - n {
        let mut pos = i;

        for j in 0..n {
            if hb[pos] != nb[j] {
                break;
            }
            pos += 1;
        }

        if pos - i == n {
            return i as i32;
        }
    }
    -1
}

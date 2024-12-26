mod tests;

pub fn is_palindroma(s: String) -> bool {
    let mut chars = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
    while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
        if a != b { return false; }
    }
    true
}


fn main() {
    println!("Hello, world!");
}

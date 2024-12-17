mod tests;

// join 会将&str汇聚成String
pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    println!("Hello, world!");
}

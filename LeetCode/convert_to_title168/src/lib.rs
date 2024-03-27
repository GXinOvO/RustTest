pub fn convert_to_title(column_number: i32) -> String {
    let mut sb = String::new();
    let mut n = column_number;
    while n > 0 {
        let pop = (n - 1) % 26;
        n = (n - 1) / 26;
        sb.push(('A' as u8 + pop as u8) as char);
    }
    sb.chars().rev().collect::<String>()
}

pub fn title_to_number(column_title: String) -> i32 {
    column_title
        .as_bytes()
        .iter()
        .fold(0, |ans, c| ans * 26 + (c - (b'A' - 1)) as i32)
}

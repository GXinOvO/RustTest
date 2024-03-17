pub fn length_of_last_word(s: String) -> i32 {
    if s.trim().len() == 0 {
        return 0;
    }

    s.split_whitespace().last().unwrap().len() as i32
}

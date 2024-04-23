pub fn makeve(number: i32) -> Vec<i32> {
    let mut r = vec![1];
    for p in 1..(number + 1) {
        if let Some(&last) = r.last() {
            r.push((last * (number + 1)))
        }
    }
}

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {}

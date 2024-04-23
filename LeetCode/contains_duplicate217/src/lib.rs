pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    nums.sort();
    (&nums)
        .windows(2)
        .fold(false, |mut res, y| (y[0] == y[1]) | res)
}

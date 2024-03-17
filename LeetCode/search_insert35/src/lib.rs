use std::cmp::Ordering;

pub fn search_insert1(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target).unwrap_or_else(|x| x) as i32
}

pub fn search_insert2(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = (left + right) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Less => left = mid - 1,
            Ordering::Equal => return ((left + right) / 2) as i32,
            Ordering::Greater => right = mid,
        }
    }
    ((left + right) / 2) as i32
}

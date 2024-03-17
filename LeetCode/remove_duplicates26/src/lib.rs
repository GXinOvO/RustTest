pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }

    let mut index = 0;
    for i in 0..nums.len() {
        if nums[i] != nums[index] {
            index += 1;
            nums[index] = nums[i];
        }
    }
    index as i32 + 1
}

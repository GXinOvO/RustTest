pub fn majority_element(nums: Vec<i32>) -> i32 {
    let (mut cnt, len, mut res) = (0, nums.len(), nums[0]);
    for i in 0..len {
        cnt += if res == nums[i] { 1 } else { -1 };
        if cnt < 0 {
            res = nums[i];
            cnt = 0;
        }
    }
    res
}

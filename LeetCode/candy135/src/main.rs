mod tests;

pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut nums = vec![1; ratings.len()];
    for i in 1..ratings.len() {
        if ratings[i] > ratings[i - 1] {
            nums[i] = nums[i - 1] + 1;
        }
    }
    ans += nums[nums.len() - 1];
    for i in (0..ratings.len() - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            nums[i] = nums[i].max(nums[i + 1] + 1);
        }
        ans += nums[i];
    }
    ans
}


fn main() {
    println!("Hello, world!");
}

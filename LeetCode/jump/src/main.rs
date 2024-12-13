
mod tests;

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut cur_right = 0;
    let mut next_right = 0;
    for i in 0..nums.len() - 1 {
        next_right = next_right.max(i as i32 + nums[i]);
        if i as i32 == cur_right {
            cur_right = next_right;
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("Hello, world!");
}

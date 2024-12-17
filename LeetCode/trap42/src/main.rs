mod tests;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut pre_max = 0;
    let mut suf_max = 0;
    while left < right {
        pre_max = pre_max.max(height[left]);
        suf_max = suf_max.max(height[right]);
        if pre_max < suf_max {
            ans += pre_max - height[left];
            left += 1;
        } else {
            ans += suf_max - height[right];
            right -= 1;
        };
    }
    ans
}

fn main() {
    println!("Hello, world!");
}

mod tests;

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut pre = vec![1; n];
    for i in 1..n {
        pre[i] = pre[i - 1] * nums[i - 1]
    }

    let mut suf = vec![1; n];

    for i in (0..n - 1).rev() {
        suf[i] = suf[i + 1] * nums[i + 1]
    }
    pre.iter().zip(suf.iter()).map(|(&p, &s)| p * s).collect()

}

fn main() {
    println!("Hello, world!");
}

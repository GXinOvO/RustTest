
mod tests;

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut mx = 0;
    for (i, &jump) in nums.iter().enumerate() {
        if i > mx { return false; }

        mx = mx.max(i + jump as usize);
    }
    true
}

fn main() {
    println!("Hello, world!");
}

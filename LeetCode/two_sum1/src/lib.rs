use std::collections::HashMap;

pub fn two_sum_1(
    nums: Vec<i32>,
    target: i32,
) -> Vec<i32>
{
    let mut out: Vec<i32> = vec![];
    for i in 0..(nums.len() - 1)
    {
        for j in i..nums.len()
        {
            if (nums[i] + nums[j]) == target
            {
                out.push(i as i32);
                out.push(j as i32);
            }
        }
    }
    out
}

pub fn two_sum_2(
    nums: Vec<i32>,
    target: i32,
) -> Vec<i32>
{
    let mut idx = HashMap::new();
    for (j, &x) in nums.iter().enumerate()
    {
        if let Some(&i) = idx.get(&(target - x))
        {
            return vec![i as i32, j as i32];
        }
        idx.insert(x, j);
    }
    unreachable!()
}
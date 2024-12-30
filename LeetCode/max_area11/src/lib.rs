pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let area = (right - left) as i32 * height[left].min(height[right]);
        ans = ans.max(area);
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49)
    }
}

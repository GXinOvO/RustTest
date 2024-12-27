pub fn two_sum(numbers: Vec<i32>, target:i32) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == target {
                result.push((i + 1) as i32);
                result.push((j + 1) as i32);
                return result;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let numbers = vec![2, 3, 4];
        let result = vec![1, 3];
        let target = 6;
        assert_eq!(two_sum(numbers, target), result);
    }
}

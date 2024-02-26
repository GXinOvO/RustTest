use two_sum1::{
    two_sum_1,
    two_sum_2,
};


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_two_sum_1()
    {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum_1(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_2()
    {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum_2(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}

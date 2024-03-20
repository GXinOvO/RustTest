use merge88::merge;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1: Vec<i32> = [1, 2, 3, 0, 0, 0].to_vec();
        let mut nums2: Vec<i32> = [2, 5, 6].to_vec();

        merge(&mut nums1, 3, &mut nums2, 3);

        let result: Vec<i32> = [1, 2, 2, 3, 5, 6].to_vec();
        assert_eq!(result, nums1)
    }
}

use search_insert35::{search_insert1, search_insert2};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        let nums1: Vec<i32> = [1, 3, 5, 6].to_vec();
        let target: i32 = 5;
        let result: i32 = search_insert1(nums1, target);
        assert_eq!(result, 2);
        // let nums2: Vec<i32> = [1, 3, 5, 6].to_vec();
        // let target: i32 = 7;
        // let result: i32 = search_insert2(nums2, target);
        // assert_eq!(result, 4)
    }
}

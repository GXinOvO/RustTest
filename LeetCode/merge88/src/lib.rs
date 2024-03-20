pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize;
    let mut n = n as usize;
    let mut right = nums1.len();

    while n > 0 {
        right -= 1;
        if m == 0 || nums1[m - 1] < nums2[n - 1] {
            nums1[right] = nums2[n - 1];
            if n > 0 {
                n -= 1;
            }
        } else {
            nums1.swap(m - 1, right);
            if m > 0 {
                m -= 1
            }
        }
    }
}

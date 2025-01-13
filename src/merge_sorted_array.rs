struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for (idx, _i) in nums1.clone().iter().enumerate() {
            if idx > m as usize - 1 {
                nums1.remove(idx);
            }
        }
        for (idx, _i) in nums2.clone().iter().enumerate() {
            if idx > n as usize - 1 {
                nums1.remove(idx);
            }
        }
    }
}

#[test]
fn test() {

}
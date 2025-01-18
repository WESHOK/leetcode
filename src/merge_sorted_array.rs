struct Solution;

impl Solution {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
        let temp = nums1[0..m as usize].to_vec();
        
        nums1.clear();
        nums1.extend(temp);
        nums1.extend(nums2.to_vec());

        nums1.sort();
    }
}

#[test]
fn test() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    let mut nums1 = vec![1];
    let m = 1;
    let mut nums2 = vec![];
    let n = 0;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1]);
    let mut nums1 = vec![0];
    let m = 0;
    let mut nums2 = vec![1];
    let n = 1;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1]);
}
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = Vec::new();

        for (idx1, i1) in nums.iter().enumerate() {
            for (idx2, i2) in nums.iter().enumerate() {
                if i1 + i2 == target && idx1 < idx2 {
                    res.extend([idx1 as i32, idx2 as i32]);
                    break;
                }
            }
        }

        res
    }
}

#[test]
fn test() {
    let nums = vec![2,7,11,15];
    let target = 9;
    let res = vec![0,1];
    assert_eq!(Solution::two_sum(nums, target), res);
    let nums = vec![3,2,4];
    let target = 6;
    let res = vec![1,2];
    assert_eq!(Solution::two_sum(nums, target), res);
    let nums = vec![3,3];
    let target = 6;
    let res = vec![0,1];
    assert_eq!(Solution::two_sum(nums, target), res);
}
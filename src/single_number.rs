struct Solution;

impl Solution {
    fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        for (idx1, i1) in nums.iter().enumerate() {
            let mut has_duplicate = false;
            for (idx2, i2) in nums.iter().enumerate() {
                if idx2 != idx1 && i2 == i1 {
                    has_duplicate = true;
                    break
                }
            }
            if !has_duplicate {
                res = *i1
            }
        }

        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 2, 1];
    let res = 1;
    assert_eq!(Solution::single_number(nums), res);
    let nums = vec![4, 1, 2, 1, 2 ];
    let res = 4;
    assert_eq!(Solution::single_number(nums), res);
    let nums = vec![1];
    let res = 1;
    assert_eq!(Solution::single_number(nums), res);
}
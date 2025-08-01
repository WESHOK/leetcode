struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut res  = String::from("");

        for i in 0..s.len() {
            for idx in i..s.len() {
                let slice = String::from(&s[i..=idx]);
                if slice == slice.chars().rev().collect::<String>() && res.len() < slice.len() {
                    res = slice
                }
            }
        }

        res
    }
}

#[test]
fn test() {
    let s = String::from("babad");
    let res = String::from("bab");
    assert_eq!(Solution::longest_palindrome(s), res);
    let s = String::from("cbbd");
    let res = String::from("bb");
    assert_eq!(Solution::longest_palindrome(s), res);
}
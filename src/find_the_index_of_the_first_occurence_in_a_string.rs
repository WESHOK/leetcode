struct Solution;

impl Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        for (i, _c) in haystack.char_indices() {
            if haystack.len() - i >= needle.len() {
                if haystack.get(i..i + needle.len()) == Some(&needle) {
                    return i.try_into().unwrap()
                }
            }
        }
        return -1
    }
}

#[test]
fn test() {
    let haystack = String::from("sadbutsad");
    let needle = String::from("sad");
    assert_eq!(Solution::str_str(haystack, needle), 0);
    let haystack = String::from("leetcode");
    let needle = String::from("leeto");
    assert_eq!(Solution::str_str(haystack, needle), -1);
    let haystack = String::from("hello");
    let needle = String::from("ll");
    assert_eq!(Solution::str_str(haystack, needle), 2);
}
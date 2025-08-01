struct Solution;

impl Solution {
    fn is_empty(strs: &Vec<String>) -> bool {
        for i in strs {
            if i.is_empty() {
                return true
            }
        }
        return false;
    }

    fn find_min(strs: &Vec<String>) -> String {
        let mut res = strs[0].clone();
        for i in strs {
            if i.len() <= res.len() {
                res = i.clone();
            }
        }
        res
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = "".to_string();

        if Solution::is_empty(&strs) {
            return res
        }

        let min = Solution::find_min(&strs);

        for (idx, i) in min.char_indices() {
            for str_ in &strs {
                if &min[..=idx] != &str_[..=idx] {
                    return res
                }
            }
            res += &i.to_string()
        }

        res
    }
}

#[test]
fn test() {
    let strs = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
    let res = "fl";
    assert_eq!(Solution::longest_common_prefix(strs), res);
    let strs = vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
    let res = "";
    assert_eq!(Solution::longest_common_prefix(strs), res);
    let strs = vec!["aaa".to_string(),"aa".to_string(),"aaa".to_string()];
    let res = "aa";
    assert_eq!(Solution::longest_common_prefix(strs), res);
}
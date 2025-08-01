struct Solution;

use util::linked_list::{ListNode, to_list};

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = vec![];

        // while let Some(node1) = l1 {
        //     node1.next = 
        // }

        res.sort();

        to_list(res)
    }
}

#[test]
fn test() {
    let l1 = to_list(vec![2,4,3]);
    let l2 = to_list(vec![2,4,3]);
    let res = to_list(vec![7,0,8]);
    assert_eq!(Solution::add_two_numbers(l1, l2), res);
    let l1 = to_list(vec![0]);
    let l2 = to_list(vec![0]);
    let res = to_list(vec![0]);
    assert_eq!(Solution::add_two_numbers(l1, l2), res);
}
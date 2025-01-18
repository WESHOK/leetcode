struct Solution;

impl Solution {
    fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n
        }
    
        let mut a = 1;
        let mut b = 2;
    
        for _ in 3..=n {
            let temp = b;
            b = a + b;
            a = temp;
        }
    
        b
    }
}

#[test]
fn test() {
    let n = 2;
    assert_eq!(Solution::climb_stairs(n), 2);
    let n = 3;
    assert_eq!(Solution::climb_stairs(n), 3);
}
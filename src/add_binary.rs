struct Solution;

impl Solution {
    fn smaller_str(a: String, b: String) -> String {
        if a.len() > b.len() {
            a
        } else {
            b
        }
    }

    fn add_binary(a: String, b: String) -> String {
        let mut inc: u32 = 0;
        let mut res = String::new();



        for ac in a.chars().rev() {
            for bc in b.chars().rev() {
                let ac_digit = ac.to_digit(10).unwrap();
                let bc_digit = bc.to_digit(10).unwrap();
                println!("{} {}", ac_digit, bc_digit);

                let sum = ac_digit + bc_digit;

                if sum == 2 {
                    inc += 1;
                }

                let res_digit = (sum + inc) % 2;
                res.insert_str(0, &res_digit.to_string());

                if inc != 0 {
                    inc -= 1;
                }
            }
        }
        return res;
    }
}

#[test]
fn test() {
    let a = String::from("11");
    let b = String::from("1");
    let res = String::from("100");
    assert_eq!(Solution::add_binary(a, b), res);
    let a = String::from("1010");
    let b = String::from("1011");
    let res = String::from("10101");
    assert_eq!(Solution::add_binary(a, b), res);
    // let a = String::from("11");
    // let b = String::from("1");
    // let res = String::from("100");
    // assert_eq!(Solution::add_binary(a, b), res);
}
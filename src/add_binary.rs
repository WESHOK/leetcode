struct Solution;

impl Solution {
    fn align_smaller_str(a: &String, b: &String) -> (String, String) {
        let mut smaller_str: String;
        let bigger_str: String;

        let diff = a.len().abs_diff(b.len());

        if a.len() > b.len() {
            smaller_str = b.clone();
            bigger_str = a.clone();
        } else {
            smaller_str = a.clone();
            bigger_str = b.clone();
        }

        smaller_str.insert_str(0, &"0".repeat(diff));

        println!("align_str: {}, {}", smaller_str, bigger_str);
        return (smaller_str, bigger_str);
    }

    fn add_binary(a: String, b: String) -> String {
        let mut inc = 0u32;
        let mut res = String::new();

        let new_str = Solution::align_smaller_str(&a, &b);

        for (bigger_c, smaller_c) in new_str.1.chars().rev().zip(new_str.0.chars().rev()) {
            println!("bigger_c: {}, smaller_c: {}", bigger_c, smaller_c);

            let bigger_digit = bigger_c.to_digit(10).unwrap();
            let smaller_digit = smaller_c.to_digit(10).unwrap();

            let sum = bigger_digit + smaller_digit + inc;

            if inc != 0 {
                inc -= 1;
            }

            let res_digit = sum % 2;
            
            res.insert_str(0, &res_digit.to_string());

            inc += sum / 2;
        }

        if inc != 0 {
            res.insert_str(0, "1");
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
}
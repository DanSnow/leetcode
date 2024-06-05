/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        use std::collections::HashMap;
        let known_map = [
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
        ]
        .into_iter()
        .collect::<HashMap<i32, &str>>();

        let mut res = vec![];

        let mut digit = 1000;

        while digit > 0 {
            let mut n = num / digit;
            let mut current = n * digit;

            match known_map.get(&current) {
                Some(s) => {
                    res.push(*s);
                }
                None => {
                    while current != 0 {
                        if n >= 5 {
                            res.push(*known_map.get(&(5 * digit)).unwrap());
                            n -= 5;
                        } else {
                            res.push(*known_map.get(&(digit)).unwrap());
                            n -= 1;
                        }
                        current = n * digit;
                    }
                }
            }
            num = num % digit;
            digit /= 10;
        }

        res.join("")
    }
}
// @lc code=end

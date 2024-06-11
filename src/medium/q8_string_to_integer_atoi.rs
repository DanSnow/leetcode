/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s.trim().as_bytes();

        if s.is_empty() {
            return 0;
        }

        let sign = match s[0] {
            b'+' => {
                s = &s[1..];
                1
            }
            b'-' => {
                s = &s[1..];
                -1
            }
            _ => 1,
        };

        let mut result: i32 = 0;

        for c in s {
            if !c.is_ascii_digit() {
                break;
            }
            let digit = (c - b'0') as i32;
            result = result.saturating_mul(10);
            result = result.saturating_add(digit * sign);
        }

        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(-42, Solution::my_atoi("   -042".to_owned()));
        assert_eq!(0, Solution::my_atoi("".to_owned()));
        assert_eq!(0, Solution::my_atoi(" ".to_owned()));
    }
}

/*
 * @lc app=leetcode id=224 lang=rust
 *
 * [224] Basic Calculator
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let mut result = 0;
        let mut stack = vec![];
        let mut sign = 1;
        let mut num = 0;
        let mut has_op = true;

        for c in s.iter().copied() {
            match c {
                b'+' => {
                    result += num * sign;
                    sign = 1;
                    num = 0;
                    has_op = true;
                }
                b'-' => {
                    if !has_op {
                        result += num * sign;
                        num = 0;
                    }
                    has_op = true;
                    sign *= -1;
                }
                b'(' => {
                    stack.push((result, sign));
                    result = 0;
                    sign = 1;
                    num = 0;
                    has_op = false;
                }
                b')' => {
                    let (origin_result, origin_sign) = stack.pop().unwrap();
                    result += num * sign;
                    result = origin_result + origin_sign * result;
                    sign = 1;
                    num = 0;
                    has_op = false;
                }
                c if c.is_ascii_digit() => {
                    num *= 10;
                    num += (c - b'0') as i32;
                    has_op = false;
                }
                _ => {
                    result += num * sign;
                    num = 0;
                    sign = 1;
                }
            }
        }

        result + num * sign
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(3, Solution::calculate("1 + 2".to_owned()));
        assert_eq!(-1, Solution::calculate("1 + -2".to_owned()));
        assert_eq!(3, Solution::calculate(" 2-1 + 2 ".to_owned()));
        assert_eq!(-1, Solution::calculate(" 2-(1 + 2) ".to_owned()));
        assert_eq!(-7, Solution::calculate("-(3 + 4)".to_owned()));
    }
}

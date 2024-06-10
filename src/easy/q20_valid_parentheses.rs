/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                }
                ')' => {
                    let x = stack.pop();
                    if x != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    let x = stack.pop();
                    if x != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    let x = stack.pop();
                    if x != Some('{') {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        stack.is_empty()
    }
}
// @lc code=end

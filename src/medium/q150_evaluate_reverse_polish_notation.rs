/*
 * @lc app=leetcode id=150 lang=rust
 *
 * [150] Evaluate Reverse Polish Notation
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut nums = vec![];
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let right = nums.pop().unwrap();
                    let left = nums.pop().unwrap();
                    nums.push(left + right);
                }
                "-" => {
                    let right = nums.pop().unwrap();
                    let left = nums.pop().unwrap();
                    nums.push(left - right);
                }
                "*" => {
                    let right = nums.pop().unwrap();
                    let left = nums.pop().unwrap();
                    nums.push(left * right);
                }
                "/" => {
                    let right = nums.pop().unwrap();
                    let left = nums.pop().unwrap();
                    nums.push(left / right);
                }
                n => {
                    nums.push(n.parse::<i32>().unwrap());
                }
            }
        }
        nums[0]
    }
}
// @lc code=end

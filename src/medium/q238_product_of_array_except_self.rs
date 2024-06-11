/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 */

use std::vec;

struct Solution;

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix_product = vec![0; nums.len()];
        let mut suffix_product = vec![0; nums.len()];

        for i in 0..nums.len() {
            prefix_product[i] = if i == 0 {
                1
            } else {
                prefix_product[i - 1] * nums[i - 1]
            };
            suffix_product[nums.len() - i - 1] = if i == 0 {
                1
            } else {
                suffix_product[nums.len() - i] * nums[nums.len() - i]
            };
        }

        let mut answer = vec![0; nums.len()];

        for i in 0..nums.len() {
            answer[i] = prefix_product[i] * suffix_product[i];
        }

        answer
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(vec![0, 1], Solution::product_except_self(vec![1, 0]));
        assert_eq!(vec![1, 0], Solution::product_except_self(vec![0, 1]));
    }
}

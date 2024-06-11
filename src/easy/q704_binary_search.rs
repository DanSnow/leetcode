/*
 * @lc app=leetcode id=704 lang=rust
 *
 * [704] Binary Search
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).map(|i| i as i32).unwrap_or(-1)
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let len = nums.len();
        let deduped = nums.into_iter().collect::<HashSet<i32>>();

        len != deduped.len()
    }
}
// @lc code=end

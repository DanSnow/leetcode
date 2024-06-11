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

        let mut seen = HashSet::new();

        for i in nums {
            if seen.contains(&i) {
                return true;
            }
            seen.insert(i);
        }

        false
    }
}
// @lc code=end

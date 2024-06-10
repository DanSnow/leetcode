/*
 * @lc app=leetcode id=1051 lang=rust
 *
 * [1051] Height Checker
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort_unstable();
        heights
            .iter()
            .copied()
            .zip(expected.iter().copied())
            .filter(|(left, right)| left != right)
            .count() as i32
    }
}
// @lc code=end

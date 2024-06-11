/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max = 0;

        while left < right {
            let area = cmp::min(height[left], height[right]) * ((right - left) as i32);
            max = cmp::max(max, area);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max
    }
}
// @lc code=end

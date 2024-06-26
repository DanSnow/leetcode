/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp;
        if nums.is_empty() {
            return 0;
        }

        let mut max = nums[0];
        let mut previous_sum = nums[0];

        for current in nums[1..].iter().copied() {
            // for each element, we only have 1 question
            previous_sum = cmp::max(
                // we start a new array
                current,
                // or we include it into previous array
                previous_sum + current,
            );
            // this is a greedy solution, as we only collect element that can create the maximum value
            max = cmp::max(previous_sum, max);
        }

        max
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 */

struct Solution;

// @lc code=start
impl Solution {
    // ref: https://www.wikiwand.com/en/Boyer%E2%80%93Moore_majority_vote_algorithm
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());

        let mut ans = nums[0];
        let mut c = 1;

        // Safety: nums shouldn't be empty
        for i in 1..nums.len() {
            if c == 0 {
                ans = nums[i];
                c = 1;
            } else if nums[i] == ans {
                c += 1;
            } else {
                c -= 1;
            }
        }

        ans
    }
}
// @lc code=end

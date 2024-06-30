/*
 * @lc app=leetcode id=594 lang=rust
 *
 * [594] Longest Harmonious Subsequence
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        use std::{cmp, collections::HashMap};
        match nums.len() {
            0 => return 0,
            1 => return 0,
            2 => {
                if (nums[0] - nums[1]).abs() != 1 {
                    return 0;
                } else {
                    return 2;
                }
            }
            _ => (),
        }

        let mut frequencies = HashMap::new();

        for num in nums.iter().copied() {
            frequencies
                .entry(num)
                .and_modify(|freq: &mut i32| {
                    *freq += 1;
                })
                .or_insert(1);
        }

        let mut max_length = 0;

        for (&num, &freq1) in frequencies.iter() {
            if let Some(freq2) = frequencies.get(&(num + 1)) {
                max_length = cmp::max(max_length, freq1 + freq2);
            }
        }

        max_length
    }
}
// @lc code=end

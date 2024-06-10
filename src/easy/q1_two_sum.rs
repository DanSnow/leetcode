/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let map = nums
            .iter()
            .copied()
            .enumerate()
            .map(|(i, val)| (val, i))
            .collect::<HashMap<i32, usize>>();

        for (index, val) in nums.iter().copied().enumerate() {
            if let Some(index2) = map.get(&(target - val)) {
                if index != *index2 {
                    return vec![index as i32, *index2 as i32];
                }
            }
        }
        unreachable!()
    }
}
// @lc code=end

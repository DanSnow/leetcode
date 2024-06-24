/*
 * @lc app=leetcode id=42 lang=rust
 *
 * [42] Trapping Rain Water
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        use std::cmp::{max, min};

        if height.len() < 2 {
            return 0;
        }

        let mut left_height = vec![0; height.len()];
        let mut right_height = vec![0; height.len() + 1];

        left_height[0] = height[0];
        right_height[height.len() - 1] = height[height.len() - 1];

        for i in 1..(height.len()) {
            left_height[i] = max(left_height[i - 1], height[i - 1]);
            right_height[height.len() - i - 1] =
                max(height[height.len() - i], right_height[height.len() - i]);
        }

        let mut total_trapping = 0;
        let mut trapping = vec![0; height.len()];
        for (i, h) in height.iter().copied().enumerate() {
            let tmp = max(min(left_height[i], right_height[i]) - h, 0);
            trapping[i] = tmp;
            total_trapping += tmp
        }

        total_trapping
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_work() {
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]))
    }
}

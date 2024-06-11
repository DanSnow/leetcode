/*
 * @lc app=leetcode id=75 lang=rust
 *
 * [75] Sort Colors
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut buckets = [0; 3];
        for i in nums.iter().copied() {
            buckets[i as usize] += 1;
        }

        let mut index = 0;
        for (color, count) in buckets.into_iter().enumerate() {
            for _ in 0..count {
                nums[index] = color as i32;
                index += 1;
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        let mut colors = vec![2, 0, 2, 1, 1, 0];

        Solution::sort_colors(&mut colors);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], colors);
    }
}

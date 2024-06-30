/*
 * @lc app=leetcode id=643 lang=rust
 *
 * [643] Maximum Average Subarray I
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        use std::cmp;
        let k = k as usize;
        assert!(k <= nums.len());
        let mut start = 0;
        let mut end = k - 1;
        let mut sum = nums[0..k].iter().copied().sum::<i32>();
        let mut max_sum = sum;
        while end < nums.len() - 1 {
            sum -= nums[start];
            start += 1;
            end += 1;
            sum += nums[end];
            max_sum = cmp::max(max_sum, sum);
        }

        f64::from(max_sum) / f64::from(k as i32)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            12.75,
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
        );
        assert_eq!(5.0, Solution::find_max_average(vec![5], 1),);
    }
}

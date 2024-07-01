/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        use std::cmp::{min, Ordering};

        if nums.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = 1;
        let mut sum = nums[0];
        let mut min_length = usize::MAX;

        while right <= nums.len() {
            match sum.cmp(&target) {
                Ordering::Greater => {
                    min_length = min(min_length, right - left);
                    sum -= nums[left];
                    left += 1;
                    if left >= right {
                        right = left + 1;
                    }
                }
                Ordering::Equal => {
                    min_length = min(min_length, right - left);
                    if right < nums.len() {
                        sum += nums[right];
                    } else {
                        break;
                    }
                    right += 1;
                    sum -= nums[left];
                    left += 1;
                }
                Ordering::Less => {
                    if right < nums.len() {
                        sum += nums[right];
                    } else {
                        break;
                    }
                    right += 1;
                }
            }
        }

        if min_length == usize::MAX {
            0
        } else {
            min_length as i32
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(2, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
        assert_eq!(
            0,
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1])
        );
        assert_eq!(3, Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]));
    }
}

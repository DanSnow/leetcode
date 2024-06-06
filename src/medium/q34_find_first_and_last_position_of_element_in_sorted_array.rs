/*
 * @lc app=leetcode id=34 lang=rust
 *
 * [34] Find First and Last Position of Element in Sorted Array
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Ok(position) => {
                let mut start = position;
                let mut end = position;
                for i in (0..position).rev() {
                    if nums[i] != target {
                        break;
                    }
                    start = i;
                }
                for i in position..nums.len() {
                    if nums[i] != target {
                        break;
                    }
                    end = i;
                }
                vec![start as i32, end as i32]
            }
            Err(_) => {
                vec![-1, -1]
            }
        }
    }
}
// @lc code=end

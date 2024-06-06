/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::{cmp::Ordering, collections::HashSet};
        nums.sort_unstable();
        let mut res = vec![];
        let mut seen = HashSet::new();
        let mut i = 0;

        while nums[i] <= 0 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            loop {
                if j >= k || k <= i {
                    break;
                }
                let ord = (nums[i] + nums[j] + nums[k]).cmp(&0);

                match ord {
                    Ordering::Equal => {
                        let group = (nums[i], nums[j], nums[k]);
                        if !seen.contains(&group) {
                            seen.insert(group);
                            res.push(vec![nums[i], nums[j], nums[k]]);
                        }
                        j += 1;
                        k = nums.len() - 1;
                    }
                    Ordering::Greater => {
                        k -= 1;
                    }
                    Ordering::Less => {
                        j += 1;
                    }
                }
            }

            i += 1;
            if i == nums.len() {
                break;
            }
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_work() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(vec![-2, 0, 1, 1, 2])
        );
    }
}

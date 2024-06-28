/*
 * @lc app=leetcode id=416 lang=rust
 *
 * [416] Partition Equal Subset Sum
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        use std::{cmp::Reverse, collections::HashMap};

        let total = nums.iter().copied().sum::<i32>();
        if total % 2 != 0 {
            return false;
        }
        let target = total / 2;

        nums.sort_unstable_by_key(|i| Reverse(*i));

        #[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
        struct UsedItem {
            n: i32,
            target: i32,
        }

        let mut memo = HashMap::new();

        fn dfs(nums: &[i32], target: i32, memo: &mut HashMap<UsedItem, bool>) -> bool {
            match nums.binary_search_by_key(&Reverse(target), |n| Reverse(*n)) {
                Ok(_) => true,
                Err(index) if index >= nums.len() => false,
                Err(index) => {
                    for i in index..nums.len() {
                        let n = nums[i];
                        match memo.get(&UsedItem { n, target }) {
                            Some(r) => {
                                if *r {
                                    return true;
                                }
                            }
                            None => {
                                let r = dfs(&nums[(i + 1)..], target - nums[i], memo);
                                memo.insert(UsedItem { n, target }, r);
                                if r {
                                    return true;
                                }
                            }
                        }
                    }
                    false
                }
            }
        }
        dfs(&nums, target, &mut memo)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(false, Solution::can_partition(vec![1, 2, 5]));
    }
}

/*
 * @lc app=leetcode id=416 lang=rust
 *
 * [416] Partition Equal Subset Sum
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        use std::{
            cmp::Reverse,
            collections::{HashMap, HashSet},
        };

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

        fn dfs(
            nums: &[i32],
            target: i32,
            memo: &mut HashMap<UsedItem, bool>,
            used: HashSet<usize>,
        ) -> bool {
            match nums.binary_search_by_key(&Reverse(target), |n| Reverse(*n)) {
                Ok(index) if !used.contains(&index) => true,
                Ok(index) if index + 1 >= nums.len() => false,
                Ok(index) => {
                    for i in (index + 1)..nums.len() {
                        if used.contains(&i) {
                            continue;
                        }
                        let n = nums[i];
                        match memo.get(&UsedItem { n, target }) {
                            Some(r) => {
                                if *r {
                                    return true;
                                }
                            }
                            None => {
                                let mut used = used.clone();
                                used.insert(i);
                                let r = dfs(nums, target - nums[i], memo, used);
                                memo.insert(UsedItem { n, target }, r);
                                if r {
                                    return true;
                                }
                            }
                        }
                    }
                    false
                }
                Err(index) if index >= nums.len() => false,
                Err(index) => {
                    for i in index..nums.len() {
                        if used.contains(&i) {
                            continue;
                        }
                        let n = nums[i];
                        match memo.get(&UsedItem { n, target }) {
                            Some(r) => {
                                if *r {
                                    return true;
                                }
                            }
                            None => {
                                let mut used = used.clone();
                                used.insert(i);
                                let r = dfs(nums, target - nums[i], memo, used);
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
        dfs(&nums, target, &mut memo, HashSet::new())
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

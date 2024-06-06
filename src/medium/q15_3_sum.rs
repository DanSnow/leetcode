/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

struct Solution;

// FIXME: tle
// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        use std::collections::HashSet;
        let mut res = vec![];
        let mut seen = HashSet::new();
        while nums.len() > 2 {
            let n = *nums.last().unwrap();
            for i in 0..nums.len() - 1 {
                let x = nums[i];
                let target = -(n + x);
                if let Some(_) = nums[(i + 1)..nums.len() - 1]
                    .iter()
                    .copied()
                    .find(|m| *m == target)
                {
                    let group = vec![x, target, n];
                    if seen.contains(&(x, target, n)) {
                        continue;
                    }
                    seen.insert((x, target, n));
                    res.push(group);
                }
            }
            while let Some(last) = nums.last() {
                if *last != n {
                    break;
                }
                nums.remove(nums.len() - 1);
            }
        }
        res
    }
}
// @lc code=end

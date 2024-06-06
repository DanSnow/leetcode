/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        candidates.sort_unstable();
        fn dfs(
            res: &mut Vec<Vec<i32>>,
            candidates: &[i32],
            path: Vec<i32>,
            target: i32,
            sum: i32,
            index: usize,
        ) {
            if sum == target {
                res.push(path);
                return;
            }
            for i in index..candidates.len() {
                let mut path = path.clone();
                let sum = sum + candidates[i];
                if sum > target {
                    break;
                }
                path.push(candidates[i]);
                dfs(res, candidates, path.clone(), target, sum, i);
            }
        }
        dfs(&mut res, &candidates, Vec::new(), target, 0, 0);
        res
    }
}
// @lc code=end

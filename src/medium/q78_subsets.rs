/*
 * @lc app=leetcode id=78 lang=rust
 *
 * [78] Subsets
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let iter = nums.iter().copied();
        let mut result = vec![];

        fn dfs(
            mut iter: impl Iterator<Item = i32> + Clone,
            result: &mut Vec<Vec<i32>>,
            mut current: Vec<i32>,
        ) {
            let mut skip_iter = iter.clone();
            skip_iter.next();

            let item = iter.next();
            if let Some(val) = item {
                dfs(skip_iter, result, current.clone());
                current.push(val);
                dfs(iter, result, current);
            } else {
                result.push(current);
            }
        }

        dfs(iter, &mut result, vec![]);

        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            vec![
                vec![],
                vec![3],
                vec![2],
                vec![2, 3],
                vec![1],
                vec![1, 3],
                vec![1, 2],
                vec![1, 2, 3]
            ],
            Solution::subsets(vec![1, 2, 3])
        );
    }
}

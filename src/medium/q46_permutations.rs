/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 */

struct Solution;

// @lc code=start
impl Solution {
    // ref: https://www.wikiwand.com/en/Heap%27s_algorithm
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }
        if nums.len() == 1 {
            return vec![nums];
        }

        let mut result = vec![];

        fn get_permute(result: &mut Vec<Vec<i32>>, nums: &mut [i32], k: usize) {
            if k == 1 {
                result.push(nums.to_vec());
            }

            for i in 0..k {
                get_permute(result, nums, k - 1);
                if k % 2 == 0 {
                    nums.swap(i, k - 1);
                } else {
                    nums.swap(0, k - 1);
                }
            }
        }

        let k = nums.len();
        get_permute(&mut result, &mut nums, k);

        result
    }
}
// @lc code=end

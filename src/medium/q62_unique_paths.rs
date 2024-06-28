/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let row = m as usize;
        let col = n as usize;
        let mut map = vec![vec![0; col]; row];

        if row == 0 || col == 0 {
            return 1;
        }

        for r in 0..row {
            map[r][0] = 1;
        }

        for c in 0..col {
            map[0][c] = 1;
        }

        for r in 1..row {
            for c in 1..col {
                map[r][c] = map[r][c - 1] + map[r - 1][c];
            }
        }

        map[row - 1][col - 1]
    }
}
// @lc code=end

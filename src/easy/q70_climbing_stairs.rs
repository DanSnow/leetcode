/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut way = vec![0; n as usize + 1];

        // the first two stair I can climb
        way[1] = 1;
        way[2] = 2;

        let n = n as usize;
        // for each star
        for i in 3..=n {
            // I either climb 1 stair from i - 1, or climb 2 stair from i - 2
            way[i] = way[i - 1] + way[i - 2];
        }

        way[n]
    }
}
// @lc code=end

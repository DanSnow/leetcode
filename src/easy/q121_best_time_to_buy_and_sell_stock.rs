/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp;
        if prices.len() < 2 {
            return 0;
        }

        let mut prev = 0;
        let mut max = 0;

        for i in 1..prices.len() {
            // previous date profit + today profit
            prev = cmp::max(prev + prices[i] - prices[i - 1], 0);
            max = cmp::max(prev, max);
        }
        max
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=322 lang=rust
 *
 * [322] Coin Change
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        use std::cmp;

        if amount == 0 {
            return 0;
        }

        let mut dp = vec![i32::MAX; amount as usize + 1];

        if coins.iter().copied().all(|coin| coin > amount) {
            return -1;
        }

        for coin in coins.iter().copied() {
            if (coin as usize) < dp.len() {
                dp[coin as usize] = 1;
            }
        }

        for i in 1..=amount {
            if coins.contains(&i) {
                dp[i as usize] = 1;
            } else {
                let i = i as usize;
                for coin in coins.iter().copied() {
                    let coin = coin as usize;
                    if i > coin && dp[i - coin] != i32::MAX {
                        dp[i] = cmp::min(dp[i - coin] + 1, dp[i]);
                    }
                }
            }
        }

        if dp[amount as usize] == i32::MAX {
            -1
        } else {
            dp[amount as usize]
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));
        assert_eq!(-1, Solution::coin_change(vec![2], 11));
    }
}

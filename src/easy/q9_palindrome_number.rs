/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let s = x.to_string().chars().collect::<Vec<char>>();
        let len = s.len();

        if len < 2 {
            return true;
        }

        // if odd, there is no need to check the mid number
        let mid = len / 2;

        (0..mid).all(|i| s[i] == s[len - i - 1])
    }
}
// @lc code=end

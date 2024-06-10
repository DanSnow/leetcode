/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = vec![];
        for c in s.chars() {
            if c.is_ascii_alphanumeric() {
                chars.push(c.to_ascii_lowercase());
            }
        }

        let mid = chars.len() / 2;
        for i in 0..mid {
            if chars[i] != chars[chars.len() - i - 1] {
                return false;
            }
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            true,
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned())
        );
        assert_eq!(false, Solution::is_palindrome("0P".to_owned()));
    }
}

/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut t = t.chars().collect::<Vec<char>>();

        s.sort_unstable();
        t.sort_unstable();

        s == t
    }
}
// @lc code=end

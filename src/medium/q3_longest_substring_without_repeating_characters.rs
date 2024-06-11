/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp;

        let s = s.as_bytes();
        if s.is_empty() {
            return 0;
        }

        let mut map = [0; 256];

        let mut start = 0;
        let mut end = 0;
        let mut max = 0;

        // the idea here is to use two pointers to keep track of the substring
        // if we find a repeated character, we move the start pointer to the right
        // else we move the end pointer to the right
        while end < s.len() {
            map[s[end] as usize] += 1;
            while map[s[end] as usize] > 1 {
                map[s[start] as usize] -= 1;
                start += 1;
            }
            max = cmp::max(max, (end - start + 1) as i32);
            end += 1;
        }

        max
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=395 lang=rust
 *
 * [395] Longest Substring with At Least K Repeating Characters
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        use std::{cmp::max, collections::HashSet};

        if s.is_empty() || k as usize > s.len() {
            return 0;
        }

        let s = s.as_bytes();

        let mut frequencies = [0; 26];

        for c in s.iter().copied() {
            frequencies[(c - b'a') as usize] += 1;
        }

        let exclude_chars = frequencies
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(index, c)| {
                if c < k {
                    Some(b'a' + (index as u8))
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        let mut count = [0; 26];
        let mut left = 0;
        let mut right = 0;
        let mut max_length = 0;

        while right < s.len() {
            let right_char = s[right];
            if exclude_chars.contains(&right_char) {
                // skip excluded characters
                right += 1;
                left = right;
                count = [0; 26];
            } else {
                // expand
                count[(s[right] - b'a') as usize] += 1;
                right += 1;
                if count.iter().copied().all(|i| i >= k || i == 0) {
                    max_length = max(max_length, (right - left) as i32);
                    // if there is some character matching the condition, we shrink the window to find if there is a match answer
                } else if count.iter().copied().any(|i| i >= k) {
                    let mut left = left;
                    let mut count = count.clone();
                    while left < right {
                        count[(s[left] - b'a') as usize] -= 1;
                        left += 1;
                        if count.iter().copied().all(|i| i >= k || i == 0) {
                            max_length = max(max_length, (right - left) as i32);
                        }
                    }
                }
            }
        }

        max_length
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring() {
        assert_eq!(3, Solution::longest_substring("aaabb".to_string(), 3));
        assert_eq!(5, Solution::longest_substring("ababbc".to_string(), 2));
        assert_eq!(1, Solution::longest_substring("a".to_string(), 1));
        assert_eq!(2, Solution::longest_substring("aa".to_string(), 1));
        assert_eq!(2, Solution::longest_substring("aa".to_string(), 2));
        assert_eq!(3, Solution::longest_substring("bbaaacbd".to_owned(), 3));
        assert_eq!(6, Solution::longest_substring("aaabbb".to_owned(), 3));
    }
}

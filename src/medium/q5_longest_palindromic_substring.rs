/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

struct Solution;

// @lc code=start
impl Solution {
    // https://cudi.dev/articles/manacher_s_algorithm_explained_with_illustrations
    // https://medium.com/hoskiss-stand/manacher-299cf75db97e
    // https://laura-neff.github.io/ManachersAlgorithm/
    pub fn longest_palindrome(s: String) -> String {
        use std::{
            cmp::{max, min, Ord, PartialOrd},
            iter,
        };

        if s.is_empty() {
            return "".to_owned();
        }

        let reversed = s.chars().rev().collect::<String>();

        if reversed == s {
            return s;
        }

        // add `#` between each char so we don't need to consider even length palindromes
        let s = s
            .chars()
            .flat_map(|c| iter::once('#').chain(iter::once(c)))
            // trailing '#'
            .chain(iter::once('#'))
            .collect::<Vec<char>>();

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct Palindrome<'a>(&'a [char]);

        impl<'a> PartialOrd for Palindrome<'a> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.len().partial_cmp(&other.0.len())
            }
        }

        impl<'a> Ord for Palindrome<'a> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.len().cmp(&other.0.len())
            }
        }

        let mut palindrome_radius = vec![0; s.len()];
        let mut longest = Palindrome(&s[1..2]);
        palindrome_radius[1] = 1;

        let mut center = 1;
        let mut right = 1;

        for i in 2..s.len() {
            // mirror the center
            if i < right {
                palindrome_radius[i] = min(palindrome_radius[2 * center - i], right - i);
            }

            while
            // left boundary
            i - palindrome_radius[i] > 0 &&
            // right boundary
            i + palindrome_radius[i] + 1 < s.len() &&
            // is palindrome
            s[i - palindrome_radius[i] - 1] == s[i + palindrome_radius[i] + 1]
            {
                // expand the center
                palindrome_radius[i] += 1;
            }

            if i + palindrome_radius[i] > right {
                center = i;
                right = i;
            }

            let palindrome =
                Palindrome(&s[(i - palindrome_radius[i])..=(i + palindrome_radius[i])]);
            longest = max(palindrome, longest);
        }

        longest
            .0
            .into_iter()
            .copied()
            .filter(|c| *c != '#')
            .collect::<String>()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!("bab", Solution::longest_palindrome("babad".to_owned()));
        assert_eq!("bb", Solution::longest_palindrome("cbbd".to_owned()));
        assert_eq!("acbbca", Solution::longest_palindrome("acbbca".to_owned()));
        assert_eq!(
            "dacbbcad",
            Solution::longest_palindrome("dacbbcad".to_owned())
        );
        assert_eq!("", Solution::longest_palindrome("".to_owned()));
        assert_eq!("a", Solution::longest_palindrome("a".to_owned()));
        assert_eq!("aa", Solution::longest_palindrome("aa".to_owned()));
        assert_eq!("aaaa", Solution::longest_palindrome("aaaa".to_owned()));
        assert_eq!(
            "xaabacxcabaax",
            Solution::longest_palindrome("xaabacxcabaaxcabaax".to_owned())
        );
    }
}

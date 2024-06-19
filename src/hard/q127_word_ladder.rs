/*
 * @lc app=leetcode id=127 lang=rust
 *
 * [127] Word Ladder
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        use std::{collections::VecDeque, mem};
        if word_list.is_empty() || !word_list.contains(&end_word) {
            return 0;
        }

        fn is_diff_by_one(a: &str, b: &str) -> bool {
            assert!(a.len() == b.len());
            let mut diff = 0;
            let a = a.as_bytes();
            let b = b.as_bytes();
            for i in 0..a.len() {
                if a[i] != b[i] {
                    diff += 1;
                    if diff > 1 {
                        return false;
                    }
                }
            }
            true
        }

        let mut queue = VecDeque::new();

        queue.push_back((begin_word, 1));

        while let Some((word, level)) = queue.pop_front() {
            for candidate in word_list.iter_mut() {
                if is_diff_by_one(&word, candidate) {
                    if candidate == &end_word {
                        return level + 1;
                    }
                    let s = mem::take(candidate);
                    queue.push_back((s, level + 1));
                }
            }

            word_list.retain(|s| !s.is_empty());
        }

        0
    }
}
// @lc code=end

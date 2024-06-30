/*
 * @lc app=leetcode id=187 lang=rust
 *
 * [187] Repeated DNA Sequences
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        use std::collections::HashSet;

        if s.len() < 10 {
            return vec![];
        }

        let mut start = 0;
        let mut end = 10;
        let mut seen = HashSet::new();
        let mut repeated_sequences = HashSet::new();

        while end < s.len() + 1 {
            let seq = &s[start..end];
            dbg!(seq);
            if !seen.insert(seq) {
                repeated_sequences.insert(seq);
            }
            start += 1;
            end += 1;
        }
        repeated_sequences
            .into_iter()
            .map(ToOwned::to_owned)
            .collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            vec!["AAAAAAAAAA".to_owned()],
            Solution::find_repeated_dna_sequences("AAAAAAAAAAA".to_owned())
        );
    }
}

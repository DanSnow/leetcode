/*
 * @lc app=leetcode id=438 lang=rust
 *
 * [438] Find All Anagrams in a String
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return vec![];
        }

        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut result = Vec::new();
        let mut map = [0; 256];
        let mut expected = [0; 256];
        for c in p {
            expected[*c as usize] += 1;
        }

        for i in 0..p.len() {
            map[s[i] as usize] += 1;
        }

        if map == expected {
            result.push(0);
        }

        for i in (0 + p.len())..s.len() {
            map[s[i] as usize] += 1;
            map[s[i - p.len()] as usize] -= 1;
            if map == expected {
                result.push((i - p.len() + 1) as i32);
            }
        }

        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            vec![0, 6],
            Solution::find_anagrams("cbaebabacd".to_owned(), "abc".to_owned())
        );
    }
}

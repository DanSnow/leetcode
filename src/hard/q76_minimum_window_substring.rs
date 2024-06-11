/*
 * @lc app=leetcode id=76 lang=rust
 *
 * [76] Minimum Window Substring
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() {
            return "".to_owned();
        }

        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut map = [0; 256];
        let mut expected = [0; 256];

        for c in t {
            expected[*c as usize] += 1;
        }

        let mut start = 0;
        let mut end = 0;
        let mut has_answer = false;
        let mut min = &s[..];

        macro_rules! is_contains {
            () => {
                map.iter()
                    .copied()
                    .enumerate()
                    .all(|(i, v)| expected[i] <= v)
            };
        }

        while end < s.len() {
            // expand end to cover all the characters in t
            while end < s.len() && !is_contains!() {
                map[s[end] as usize] += 1;
                end += 1;
            }
            // impossible to find a window with all the characters in t
            if !is_contains!() {
                break;
            }

            has_answer = true;

            // shrink the start pointer to find the smallest window

            while start < end && is_contains!() {
                if min.len() > (end - start) {
                    min = &s[start..end];
                }
                map[s[start] as usize] -= 1;
                start += 1;
            }
        }

        if has_answer {
            String::from_utf8_lossy(min).into_owned()
        } else {
            "".to_owned()
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            "BANC",
            Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned())
        );
        assert_eq!("a", Solution::min_window("ab".to_owned(), "a".to_owned()));
        assert_eq!(
            "abc",
            Solution::min_window("abc".to_owned(), "ac".to_owned())
        );
    }
}

/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        use std::collections::HashMap;
        use std::sync::OnceLock;
        static DIGIT_MAP: OnceLock<HashMap<char, &[char]>> = OnceLock::new();
        let digit_map = [
            ('2', ['a', 'b', 'c'].as_slice()),
            ('3', ['d', 'e', 'f'].as_slice()),
            ('4', ['g', 'h', 'i'].as_slice()),
            ('5', ['j', 'k', 'l'].as_slice()),
            ('6', ['m', 'n', 'o'].as_slice()),
            ('7', ['p', 'q', 'r', 's'].as_slice()),
            ('8', ['t', 'u', 'v'].as_slice()),
            ('9', ['w', 'x', 'y', 'z'].as_slice()),
        ]
        .into_iter()
        .collect::<HashMap<char, &[char]>>();
        let _ = DIGIT_MAP.set(digit_map);

        let chars = digits.chars().collect::<Vec<char>>();
        let mut res = vec![];

        fn find_all(res: &mut Vec<String>, path: String, chars: &[char], index: usize) {
            if index == chars.len() {
                if !path.is_empty() {
                    res.push(path);
                }
                return;
            }

            let candidates = DIGIT_MAP.get().unwrap().get(&chars[index]).unwrap();
            for c in candidates.iter().copied() {
                let mut current = path.clone();
                current.push(c);
                find_all(res, current, chars, index + 1);
            }
        }

        find_all(&mut res, String::new(), &chars, 0);
        res
    }
}
// @lc code=end

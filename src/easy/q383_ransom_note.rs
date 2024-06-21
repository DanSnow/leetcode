/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut bucket = [0; 26];
        let ransom_note = ransom_note.as_bytes();
        let magazine = magazine.as_bytes();

        for c in magazine.iter().copied() {
            bucket[(c - b'a') as usize] += 1;
        }

        for c in ransom_note.iter().copied() {
            if bucket[(c - b'a') as usize] == 0 {
                return false;
            }
            bucket[(c - b'a') as usize] -= 1;
        }
        true
    }
}
// @lc code=end

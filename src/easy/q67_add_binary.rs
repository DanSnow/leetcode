/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut left = a.iter().copied().rev();
        let mut right = b.iter().copied().rev();
        let mut carry = 0;
        let len = a.len().max(b.len());
        let mut ans = vec![];

        for _ in 0..len {
            let l = match left.next() {
                Some(b'0') => 0,
                Some(b'1') => 1,
                None => 0,
                _ => unreachable!(),
            };
            let r = match right.next() {
                Some(b'0') => 0,
                Some(b'1') => 1,
                None => 0,
                _ => unreachable!(),
            };
            let c = if (carry + l + r) % 2 == 0 { '0' } else { '1' };
            ans.push(c);
            carry = (carry + l + r) / 2;
        }
        if carry == 1 {
            ans.push('1');
        }
        ans.reverse();
        ans.into_iter().collect()
    }
}
// @lc code=end

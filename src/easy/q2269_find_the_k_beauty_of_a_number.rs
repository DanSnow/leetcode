/*
 * @lc app=leetcode id=2269 lang=rust
 *
 * [2269] Find the K-Beauty of a Number
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let num_str = num.to_string();
        assert!((k as usize) <= num_str.len());
        let mut count = 0;
        let mut position = k as usize;
        let mut n = num_str[0..position].parse::<i32>().unwrap();

        while position <= num_str.len() {
            if n != 0 && num % n == 0 {
                count += 1;
            }

            n %= 10_i32.pow((k - 1) as u32);
            n *= 10;
            if position < num_str.len() {
                n += (num_str.as_bytes()[position] - b'0') as i32;
            }

            position += 1;
        }

        count
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor_substrings() {
        assert_eq!(Solution::divisor_substrings(240, 2), 2);
        assert_eq!(Solution::divisor_substrings(430043, 2), 2);
        assert_eq!(Solution::divisor_substrings(2, 1), 1);
        assert_eq!(Solution::divisor_substrings(65, 1), 1);
    }
}

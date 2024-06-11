/*
 * @lc app=leetcode id=278 lang=rust
 *
 * [278] First Bad Version
 */

struct Solution;

// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid;
            }
            if right - left == 1 {
                if self.isBadVersion(left) {
                    return left;
                } else {
                    return right;
                }
            }
        }
        left
    }
}
// @lc code=end

impl Solution {
    fn isBadVersion(&self, version: i32) -> bool {
        version >= 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        let solution = Solution;
        assert_eq!(4, solution.first_bad_version(5));
    }
}

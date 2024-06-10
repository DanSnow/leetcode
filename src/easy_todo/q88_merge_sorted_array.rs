/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

struct Solution;

// FIXME: WA
// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        use std::mem;

        if n == 0 {
            return;
        }

        let mut j = 0;
        for i in 0..(m as usize) {
            if nums1[i] > nums2[j] {
                mem::swap(&mut nums1[i], &mut nums2[j]);
            }
            if nums2[j] > nums2[(j + 1) % nums2.len()] {
                j += 1;
                j %= nums2.len();
            }
        }
        if j == 0 {
            for i in 0..(n as usize) {
                nums1[(m as usize) + i] = nums2[i];
            }
        } else {
            let mut part1 = nums2[0..j].iter().copied().peekable();
            let mut part2 = nums2[j..].iter().copied().peekable();
            let mut i = m as usize;
            loop {
                match (part1.peek(), part2.peek()) {
                    (Some(a), Some(b)) if b > a => {
                        nums1[i] = part2.next().unwrap();
                    }
                    (Some(_), Some(_)) => {
                        nums1[i] = part1.next().unwrap();
                    }
                    (None, Some(_)) => {
                        nums1[i] = part2.next().unwrap();
                    }
                    (Some(_), None) => {
                        nums1[i] = part1.next().unwrap();
                    }
                    (None, None) => break,
                }
                i += 1;
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        // let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        // let mut nums2 = vec![2, 5, 6];
        // Solution::merge(&mut nums1, 3, &mut nums2, 3);
        // assert_eq!([1, 2, 2, 3, 5, 6].as_slice(), nums1.as_slice());

        let mut nums1 = vec![1, 5, 6, 0, 0, 0];
        let mut nums2 = vec![2, 3, 3];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!([1, 2, 3, 3, 5, 6].as_slice(), nums1.as_slice());

        // let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        // let mut nums2 = vec![1, 2, 3];
        // Solution::merge(&mut nums1, 3, &mut nums2, 3);
        // assert_eq!([1, 2, 3, 4, 5, 6].as_slice(), nums1.as_slice());

        // let mut nums1 = vec![6, 0, 0, 0, 0, 0];
        // let mut nums2 = vec![1, 2, 3, 4, 5];
        // Solution::merge(&mut nums1, 1, &mut nums2, 5);
        // assert_eq!([1, 2, 3, 4, 5, 6].as_slice(), nums1.as_slice());
    }
}

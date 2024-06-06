/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        // do linear search if length is smaller then 3
        if nums.len() < 3 {
            for i in 0..nums.len() {
                if nums[i] == target {
                    return i as i32;
                }
            }
            return -1;
        }

        // it's sorted, do normal binary search
        if nums[0] < nums[nums.len() - 1] {
            return nums.binary_search(&target).map(|x| x as i32).unwrap_or(-1);
        }

        let first = nums[0];
        // find the rotated position
        let split_position = nums
            .binary_search_by(|x| match first.cmp(x) {
                Ordering::Equal | Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
            })
            .unwrap_err();

        match target.cmp(&first) {
            Ordering::Equal => 0,
            // If it's larger then the first number, target should be in the left part
            Ordering::Greater => nums[0..split_position]
                .binary_search(&target)
                .map(|x| x as i32)
                .unwrap_or(-1),
            Ordering::Less => nums[split_position..]
                .binary_search(&target)
                .map(|x| (x + split_position) as i32)
                .unwrap_or(-1),
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
        assert_eq!(3, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 7));
        assert_eq!(0, Solution::search(vec![3, 1], 3));
        assert_eq!(0, Solution::search(vec![5, 1, 3], 5));
    }
}

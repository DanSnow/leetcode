/*
 * @lc app=leetcode id=56 lang=rust
 *
 * [56] Merge Intervals
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::mem;

        fn is_not_overlap<'a>(mut interval1: &'a Vec<i32>, mut interval2: &'a Vec<i32>) -> bool {
            // we must check the order
            if interval1[0] > interval2[0] {
                mem::swap(&mut interval1, &mut interval2);
            }
            // interval2 is before interval1
            if interval1[0] > interval2[1] {
                return true;
            }
            // interval2 is after interval1
            if interval1[1] < interval2[0] {
                return true;
            }

            false
        }

        fn merge_interval(target_interval: &mut Vec<i32>, interval: Vec<i32>) {
            assert!(!is_not_overlap(target_interval, &interval));

            if target_interval[0] > interval[0] {
                target_interval[0] = interval[0];
            }

            if interval[1] > target_interval[1] {
                target_interval[1] = interval[1];
            }
        }

        intervals.sort_unstable_by_key(|interval| interval[0]);

        // merge overlap
        let mut merged_intervals = vec![];
        for interval in intervals {
            match merged_intervals.last_mut() {
                Some(last_interval) => {
                    if is_not_overlap(last_interval, &interval) {
                        merged_intervals.push(interval);
                    } else {
                        merge_interval(last_interval, interval);
                    }
                }
                None => {
                    merged_intervals.push(interval);
                }
            }
        }

        merged_intervals
    }
}
// @lc code=end

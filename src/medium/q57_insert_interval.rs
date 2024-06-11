/*
 * @lc app=leetcode id=57 lang=rust
 *
 * [57] Insert Interval
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        use std::mem;

        if intervals.len() == 0 {
            return vec![new_interval];
        }

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

        // find a insert point with binary search
        let insert_point =
            match intervals.binary_search_by_key(&new_interval[0], |interval| interval[0]) {
                Ok(i) => i,
                Err(i) => i,
            };
        intervals.insert(insert_point, new_interval);

        // merge overlap
        let mut inserted_intervals = vec![];
        for interval in intervals {
            match inserted_intervals.last_mut() {
                Some(last_interval) => {
                    if is_not_overlap(last_interval, &interval) {
                        inserted_intervals.push(interval);
                    } else {
                        merge_interval(last_interval, interval);
                    }
                }
                None => {
                    inserted_intervals.push(interval);
                }
            }
        }

        inserted_intervals
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            vec![vec![1, 5], vec![6, 9]],
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
        );
    }
}

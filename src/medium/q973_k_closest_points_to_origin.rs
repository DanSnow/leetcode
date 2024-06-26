/*
 * @lc app=leetcode id=973 lang=rust
 *
 * [973] K Closest Points to Origin
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        use std::cmp::{Ord, Ordering, PartialOrd};
        #[derive(Debug, Clone, Eq, PartialEq)]
        struct Point {
            point: Vec<i32>,
            distance: i32,
        }

        impl PartialOrd for Point {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.distance.partial_cmp(&other.distance)
            }
        }

        impl Ord for Point {
            fn cmp(&self, other: &Self) -> Ordering {
                self.distance.cmp(&other.distance)
            }
        }

        impl Point {
            fn new(point: Vec<i32>) -> Self {
                let distance = point[0].pow(2) + point[1].pow(2);
                Self { point, distance }
            }
        }

        let mut points = points.into_iter().map(Point::new).collect::<Vec<_>>();
        points.sort_unstable();

        points.truncate(k as usize);

        points
            .into_iter()
            .map(|Point { point, .. }| point)
            .collect()
    }
}
// @lc code=end

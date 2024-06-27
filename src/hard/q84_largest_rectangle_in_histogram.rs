/*
 * @lc app=leetcode id=84 lang=rust
 *
 * [84] Largest Rectangle in Histogram
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        if heights.is_empty() {
            return 0;
        }
        let mut current_height = heights[0];
        let mut max_rectangle = heights[0];
        let mut width = 1;

        for height in heights[1..].iter().copied() {
            current_height = min(height, current_height);
            let rectangle_with_current_bar = (width + 1) * current_height;
            let rectangle_from_scratch = 1 * height;
            dbg!(
                current_height,
                height,
                width,
                rectangle_from_scratch,
                rectangle_with_current_bar
            );
            if rectangle_from_scratch >= rectangle_with_current_bar {
                width = 1;
                max_rectangle = max(max_rectangle, rectangle_from_scratch);
                current_height = height;
            } else {
                width += 1;
                max_rectangle = max(rectangle_with_current_bar, max_rectangle);
            }
        }

        max_rectangle
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        // assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
        assert_eq!(9, Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5]));
    }
}

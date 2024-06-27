/*
 * @lc app=leetcode id=84 lang=rust
 *
 * [84] Largest Rectangle in Histogram
 */

struct Solution;

// @lc code=start
impl Solution {
    // ref: https://medium.com/algorithms-digest/largest-rectangle-in-histogram-234004ecd15a
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        use std::cmp::max;
        if heights.is_empty() {
            return 0;
        }

        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        struct Bar {
            height: i32,
            index: i32,
        }

        // the idea here is we need to find the previous smaller element from the left side
        // so we can use previous element's index + 1 to make the largest area until current position
        // ref: https://medium.com/algorithms-digest/previous-smaller-element-e3996fb8be3c

        // we add a extra element to ensure that we can correctly handle the last item.
        let mut left_side_smaller_elements: Vec<Bar> = vec![Bar {
            height: -1,
            index: -1,
        }];

        let mut max_area = heights[0];

        for (i, h) in heights.iter().copied().enumerate() {
            let i = i as i32;
            // pop all bars that are taller than current bar
            // we don't need to check the empty as the `let` also check for it
            while let Some(Bar { height, .. }) = left_side_smaller_elements.last().copied() {
                if height < h {
                    break;
                }

                // pop the bar
                let Bar { height, .. } = left_side_smaller_elements.pop().unwrap();

                // calculate the area with "i" as the right boundary (exclusive) of the bar
                let width = i
                    - left_side_smaller_elements
                        .last()
                        .map(|b| b.index)
                        .unwrap_or(0)
                    - 1;
                max_area = max(max_area, width * height);
            }
            left_side_smaller_elements.push(Bar {
                index: i,
                height: h,
            });
        }

        // pop the remaining bars
        // now we have the smaller elements from the left side
        // which mean all the element should be safe to extend to right side
        while let Some(Bar { height, .. }) = left_side_smaller_elements.pop() {
            // need to find the previous one to create the largest area
            let width = heights.len() as i32
                - left_side_smaller_elements
                    .last()
                    .map(|b| b.index)
                    .unwrap_or(0)
                - 1;
            max_area = max_area.max(width * height);
        }

        max_area
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
        assert_eq!(9, Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5]));
        assert_eq!(2, Solution::largest_rectangle_area(vec![1, 1]));
    }
}

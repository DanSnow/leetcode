/*
 * @lc app=leetcode id=54 lang=rust
 *
 * [54] Spiral Matrix
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let placeholder = i32::MAX;
        let mut result = vec![];

        let mut direction_iter = directions.iter().cycle();
        let width = matrix[0].len();
        let height = matrix.len();
        let mut row = 0;
        let mut col = 0;

        if width == 0 && height == 0 {
            return result;
        }

        let mut current_direction = direction_iter.next().unwrap();

        result.push(matrix[row][col]);
        matrix[row][col] = placeholder;

        loop {
            let r = row as i32;
            let c = col as i32;
            let mut next_r = r + current_direction.0;
            let mut next_c = c + current_direction.1;
            let mut loop_count = 0;
            while loop_count < 4
                && (next_c < 0
                    || next_c >= width as i32
                    || next_r < 0
                    || next_r >= height as i32
                    || matrix[next_r as usize][next_c as usize] == placeholder)
            {
                current_direction = direction_iter.next().unwrap();
                next_r = r + current_direction.0;
                next_c = c + current_direction.1;
                loop_count += 1;
            }
            if loop_count >= 4 {
                break;
            }
            result.push(matrix[next_r as usize][next_c as usize]);
            matrix[next_r as usize][next_c as usize] = placeholder;
            row = next_r as usize;
            col = next_c as usize;
        }
        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
    }
}

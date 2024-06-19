/*
 * @lc app=leetcode id=994 lang=rust
 *
 * [994] Rotting Oranges
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::{
            cmp::{max, min},
            collections::VecDeque,
        };

        let width = grid[0].len() as i32;
        let height = grid.len() as i32;
        let max_distance = width * height + 1;

        let row_bound = 0..height;
        let col_bound = 0..width;

        let mut distances = vec![vec![max_distance; grid[0].len()]; grid.len()];

        let mut max_rotten_days = 0;

        let mut queue = VecDeque::new();

        // fill rotten orange distance to 0
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 2 {
                    queue.push_back((r as i32, c as i32));
                    distances[r as usize][c as usize] = 0;
                }
            }
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((r, c)) = queue.pop_front() {
            let mut distance = distances[r as usize][c as usize];
            grid[r as usize][c as usize] = 2;
            for direction in directions.iter() {
                let next_r = r + direction.0;
                let next_c = c + direction.1;
                if col_bound.contains(&next_c) && row_bound.contains(&next_r) {
                    distance = min(distance, distances[next_r as usize][next_c as usize] + 1);
                    if grid[next_r as usize][next_c as usize] == 1 {
                        queue.push_back((next_r, next_c));
                    }
                }
            }
            distances[r as usize][c as usize] = distance;
            if distance < max_distance {
                max_rotten_days = max(distance, max_rotten_days);
            }
        }

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    return -1;
                }
            }
        }

        max_rotten_days
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            -1,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]])
        );
        assert_eq!(
            58,
            Solution::oranges_rotting(vec![
                vec![2, 0, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 1, 0, 1, 1, 1, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 1, 0, 1, 0, 1],
                vec![1, 0, 1, 0, 0, 0, 0, 1, 0, 1],
                vec![1, 0, 1, 1, 1, 1, 1, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            ])
        );
    }
}

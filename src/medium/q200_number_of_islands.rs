/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        use std::ops::Range;
        let mut count = 0;
        if grid.is_empty() {
            return 0;
        }

        if grid[0].is_empty() {
            return 0;
        }

        assert!(grid.len() > 0 && grid[0].len() > 0);

        // ! no idea why BFS is MLE, so we choose DFS instead
        fn clear_island(
            grid: &mut Vec<Vec<char>>,
            row_bound: Range<i32>,
            col_bound: Range<i32>,
            row: i32,
            col: i32,
        ) {
            static DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            grid[row as usize][col as usize] = '0';
            for direction in DIRECTIONS.iter() {
                let row = row + direction.0;
                let col = col + direction.1;
                if row_bound.contains(&row)
                    && col_bound.contains(&col)
                    && grid[row as usize][col as usize] == '1'
                {
                    clear_island(grid, row_bound.clone(), col_bound.clone(), row, col);
                }
            }
        }

        let height = grid.len();
        let width = grid[0].len();
        let row_bound = 0..height as i32;
        let col_bound = 0..width as i32;

        for row in 0..height {
            for col in 0..width {
                if grid[row][col] == '1' {
                    clear_island(
                        &mut grid,
                        row_bound.clone(),
                        col_bound.clone(),
                        row as i32,
                        col as i32,
                    );
                    count += 1;
                }
            }
        }
        count
    }
}
// @lc code=end

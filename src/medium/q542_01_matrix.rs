/*
 * @lc app=leetcode id=542 lang=rust
 *
 * [542] 01 Matrix
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::min;
        let width = mat[0].len() as i32;
        let height = mat.len() as i32;
        let max_distance = width + height;

        let mut distances = vec![vec![max_distance; mat[0].len()]; mat.len()];

        // fill zero cells to 0
        for r in 0..mat.len() {
            for c in 0..mat[0].len() {
                if mat[r][c] == 0 {
                    distances[r as usize][c as usize] = 0;
                }
            }
        }

        // process from top left to bottom right
        for r in 0..mat.len() {
            for c in 0..mat[0].len() {
                if mat[r][c] == 1 {
                    let r = r as i32;
                    let c = c as i32;
                    let top = if r > 0 {
                        distances[(r - 1) as usize][c as usize]
                    } else {
                        max_distance
                    };
                    let left = if c > 0 {
                        distances[r as usize][(c - 1) as usize]
                    } else {
                        max_distance
                    };
                    distances[r as usize][c as usize] =
                        min(min(left, top) + 1, distances[r as usize][c as usize]);
                }
            }
        }

        // process from bottom right to top left
        for r in (0..mat.len()).rev() {
            for c in (0..mat[0].len()).rev() {
                if mat[r][c] == 1 {
                    let r = r as i32;
                    let c = c as i32;
                    let bottom = if r < height - 1 {
                        distances[(r + 1) as usize][c as usize]
                    } else {
                        max_distance
                    };
                    let right = if c < width - 1 {
                        distances[r as usize][(c + 1) as usize]
                    } else {
                        max_distance
                    };
                    distances[r as usize][c as usize] =
                        min(min(bottom, right) + 1, distances[r as usize][c as usize]);
                }
            }
        }

        distances
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
        );

        assert_eq!(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]],
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]])
        );

        assert_eq!(
            vec![vec![2, 1, 0], vec![1, 0, 0], vec![0, 0, 0]],
            Solution::update_matrix(vec![vec![1, 1, 0], vec![1, 0, 0], vec![0, 0, 0]])
        );
    }
}

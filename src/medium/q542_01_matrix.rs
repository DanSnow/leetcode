/*
 * @lc app=leetcode id=542 lang=rust
 *
 * [542] 01 Matrix
 */
struct Solution;

// @lc code=start
impl Solution {
    // FIXME: TLE on large test case
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::{
            cmp::min,
            collections::{HashSet, VecDeque},
        };
        let mut distances = vec![vec![i32::MAX; mat[0].len()]; mat.len()];

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    distances[i][j] = 0;
                }
            }
        }

        for r in 0..mat.len() {
            for c in 0..mat[0].len() {
                if mat[r][c] == 1 {
                    let r = r as i32;
                    let c = c as i32;
                    let mut queue = VecDeque::from(vec![
                        (r - 1, c, 1),
                        (r + 1, c, 1),
                        (r, c - 1, 1),
                        (r, c + 1, 1),
                    ]);
                    let mut seen = HashSet::new();

                    let current_row = r as usize;
                    let current_col = c as usize;
                    // bfs to find the shortest path
                    while let Some((r, c, d)) = queue.pop_front() {
                        if seen.contains(&(r, c)) {
                            continue;
                        }
                        seen.insert((r, c));

                        // it's obviously not possible to have a shorter path
                        if distances[current_row][current_col] < d {
                            continue;
                        }

                        if r >= 0 && r < mat.len() as i32 && c >= 0 && c < mat[0].len() as i32 {
                            if distances[r as usize][c as usize] == i32::MAX {
                                queue.push_back((r + 1, c, d + 1));
                                queue.push_back((r - 1, c, d + 1));
                                queue.push_back((r, c + 1, d + 1));
                                queue.push_back((r, c - 1, d + 1));
                            } else {
                                distances[current_row][current_col] = min(
                                    distances[r as usize][c as usize].saturating_add(d),
                                    distances[current_row][current_col],
                                );
                            }
                        }
                    }
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

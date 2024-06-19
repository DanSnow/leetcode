/*
 * @lc app=leetcode id=79 lang=rust
 *
 * [79] Word Search
 */

struct Solution;

// @lc code=start
impl Solution {
    // FIXME: should use dfs
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        use std::collections::{HashSet, VecDeque};

        if word.is_empty() {
            return true;
        }

        if board.is_empty() || board[0].is_empty() {
            return false;
        }

        let width = board[0].len();
        let height = board.len();
        let row_bound = 0..height as i32;
        let col_bound = 0..width as i32;
        let word = word.chars().collect::<Vec<char>>();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for row in 0..height {
            for col in 0..width {
                if board[row][col] == word[0] {
                    let mut queue = VecDeque::new();
                    queue.push_back((row as i32, col as i32, 0, HashSet::new()));

                    while let Some((r, c, i, seen)) = queue.pop_front() {
                        if seen.contains(&(r, c)) {
                            continue;
                        }
                        if word[i] == board[r as usize][c as usize] {
                            if i == word.len() - 1 {
                                return true;
                            }
                            let mut seen = seen.clone();
                            seen.insert((r, c));
                            for direction in directions.iter() {
                                let next_r = r + direction.0;
                                let next_c = c + direction.1;
                                if col_bound.contains(&next_c)
                                    && row_bound.contains(&next_r)
                                    && !seen.contains(&(next_r, next_c))
                                {
                                    queue.push_back((next_r, next_c, i + 1, seen.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            true,
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_owned()
            )
        );
        assert_eq!(
            true,
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'E', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCESEEEFS".to_owned()
            )
        );
        assert_eq!(
            true,
            Solution::exist(vec![vec!['a', 'b'], vec!['c', 'd'],], "abdc".to_owned())
        );
    }
}

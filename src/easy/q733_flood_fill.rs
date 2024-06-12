/*
 * @lc app=leetcode id=733 lang=rust
 *
 * [733] Flood Fill
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let original = image[sr as usize][sc as usize];

        if original == color {
            return image;
        }

        fn fill(image: &mut Vec<Vec<i32>>, r: i32, c: i32, original: i32, color: i32) {
            if r < 0 || r >= image.len() as i32 || c < 0 || c >= image[0].len() as i32 {
                return;
            }

            if image[r as usize][c as usize] == original {
                image[r as usize][c as usize] = color;
                fill(image, r - 1, c, original, color);
                fill(image, r + 1, c, original, color);
                fill(image, r, c - 1, original, color);
                fill(image, r, c + 1, original, color);
            }
        }

        fill(&mut image, sr, sc, original, color);

        image
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2)
        );
    }
}

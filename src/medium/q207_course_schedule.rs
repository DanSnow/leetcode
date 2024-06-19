/*
 * @lc app=leetcode id=207 lang=rust
 *
 * [207] Course Schedule
 */

struct Solution;

// @lc code=start
impl Solution {
    // ref: https://web.ntnu.edu.tw/~algo/DirectedAcyclicGraph.html
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        use std::collections::HashMap;

        if prerequisites.is_empty() {
            return true;
        }

        let mut graph = HashMap::new();
        let mut references = vec![0; num_courses as usize];

        for edge in prerequisites.iter() {
            references[edge[1] as usize] += 1;
            graph
                .entry(edge[0])
                .and_modify(|list: &mut Vec<i32>| {
                    list.push(edge[1]);
                })
                .or_insert_with(|| vec![edge[1]]);
        }

        for _ in 0..num_courses as usize {
            let start_node = match references
                .iter()
                .copied()
                .enumerate()
                .find(|(_, r)| *r == 0)
            {
                Some((start_node, _)) => start_node,
                None => return false,
            };

            if start_node == usize::MAX {
                return false;
            }

            references[start_node] = -1;

            if let Some(connected_nodes) = graph.get(&(start_node as i32)) {
                for node in connected_nodes.iter().copied() {
                    references[node as usize] -= 1;
                }
            }
        }

        true
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_finish() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
        assert!(Solution::can_finish(2, vec![vec![0, 1]]));
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
}

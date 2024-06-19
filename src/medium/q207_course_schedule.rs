/*
 * @lc app=leetcode id=207 lang=rust
 *
 * [207] Course Schedule
 */

struct Solution;

// @lc code=start
impl Solution {
    // ref: https://web.ntnu.edu.tw/~algo/DirectedAcyclicGraph.html
    // Kahn's algorithm
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        use std::collections::{HashMap, VecDeque};

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

        let mut queue = references
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(node, r)| if r == 0 { Some(node as i32) } else { None })
            .collect::<VecDeque<_>>();

        let mut visited_count = 0;
        while let Some(start_node) = queue.pop_front() {
            if let Some(connected_nodes) = graph.get(&start_node) {
                for node in connected_nodes.iter().copied() {
                    references[node as usize] -= 1;
                    if references[node as usize] == 0 {
                        queue.push_back(node);
                    }
                }
            }

            visited_count += 1;
        }

        visited_count == num_courses
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

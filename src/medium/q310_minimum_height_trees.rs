/*
 * @lc app=leetcode id=310 lang=rust
 *
 * [310] Minimum Height Trees
 */

struct Solution;

// @lc code=start
impl Solution {
    // ref: https://leetcode.com/problems/minimum-height-trees/solutions/5060851/python3-o-n-time-and-space-apr-23-2024-daily/
    pub fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;

        if n == 1 {
            return vec![0];
        }

        if edges.is_empty() {
            return vec![0];
        }

        let mut graph = HashMap::new();

        for edge in edges.iter() {
            graph
                .entry(edge[0])
                .and_modify(|list: &mut Vec<i32>| {
                    list.push(edge[1]);
                })
                .or_insert_with(|| vec![edge[1]]);

            graph
                .entry(edge[1])
                .and_modify(|list: &mut Vec<i32>| {
                    list.push(edge[0]);
                })
                .or_insert_with(|| vec![edge[0]]);
        }

        // find all the leaf nodes
        let mut leaves = graph
            .iter()
            .filter_map(|(k, v)| if v.len() == 1 { Some(*k) } else { None })
            .collect::<Vec<i32>>();

        while n > 2 {
            n -= leaves.len() as i32;
            let mut next_leaves = Vec::new();
            // we keep remove the leaf nodes from the graph
            for leaf in leaves.iter() {
                let next_node = graph[leaf][0];
                graph
                    .get_mut(&next_node)
                    .unwrap()
                    .retain(|&node| node != *leaf);
                if graph[&next_node].len() == 1 {
                    next_leaves.push(next_node);
                }
            }
            leaves = next_leaves;
        }
        // until there are only 1~2 nodes in the graph, we found the gravity center nodes
        leaves
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![1],
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]])
        );
    }
}

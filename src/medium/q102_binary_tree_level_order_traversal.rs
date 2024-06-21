/*
 * @lc app=leetcode id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
 */

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        if root.is_none() {
            return vec![];
        }

        queue.push_back((root.clone().unwrap(), 0));

        while let Some((node, level)) = queue.pop_front() {
            while level >= result.len() {
                result.push(vec![]);
            }
            let node = node.borrow();
            result[level].push(node.val);

            if let Some(left) = &node.left {
                queue.push_back((left.clone(), level + 1));
            }

            if let Some(right) = &node.right {
                queue.push_back((right.clone(), level + 1));
            }
        }
        result
    }
}
// @lc code=end

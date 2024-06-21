/*
 * @lc app=leetcode id=199 lang=rust
 *
 * [199] Binary Tree Right Side View
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
    // right side view is the level order with the rightmost node in each level.
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        if root.is_none() {
            return vec![];
        }

        queue.push_back((root.clone().unwrap(), 0));

        while let Some((node, level)) = queue.pop_front() {
            while level >= result.len() {
                // anything is ok, we just need to allocate the space
                result.push(0);
            }
            let node = node.borrow();
            // we are doing bfs, so the rightmost node will be the last one in the queue
            result[level] = node.val;

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

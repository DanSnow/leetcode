/*
 * @lc app=leetcode id=235 lang=rust
 *
 * [235] Lowest Common Ancestor of a Binary Search Tree
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(p) = p else {
            return None;
        };
        let Some(q) = q else {
            return None;
        };

        let p_ref = p.borrow();
        let q_ref = q.borrow();

        let (large, small) = if p_ref.val >= q_ref.val {
            (p_ref, q_ref)
        } else {
            (q_ref, p_ref)
        };

        match root {
            None => None,
            Some(node) => {
                let node_ref = node.borrow();
                // All the left children should smaller than p and q
                if node_ref.val > large.val {
                    drop(large);
                    drop(small);
                    Self::lowest_common_ancestor(node_ref.left.clone(), Some(p), Some(q))
                // All the right children should larger than p and q
                } else if node_ref.val < small.val {
                    drop(large);
                    drop(small);
                    Self::lowest_common_ancestor(node_ref.right.clone(), Some(p), Some(q))
                } else {
                    drop(node_ref);
                    Some(node)
                }
            }
        }
    }
}
// @lc code=end

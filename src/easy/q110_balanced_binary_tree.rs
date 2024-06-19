/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
 */

struct Solution;

// Definition for a binary tree node.
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        use std::cmp::max;

        fn height(node: Rc<RefCell<TreeNode>>) -> i32 {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                return 1;
            }
            let left_height = node.left.as_ref().map_or(0, |n| height(n.clone()));
            let right_height = node.right.as_ref().map_or(0, |n| height(n.clone()));
            max(left_height, right_height) + 1
        }

        match root {
            None => true,
            Some(root) => {
                let root = root.borrow();
                let left_height = root.left.as_ref().map_or(0, |n| height(n.clone()));
                let right_height = root.right.as_ref().map_or(0, |n| height(n.clone()));
                (left_height - right_height).abs() <= 1
                // to check if the tree is balanced, we need to check if the left and right subtree are balanced
                    && Self::is_balanced(root.left.clone())
                    && Self::is_balanced(root.right.clone())
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::is_balanced(None));
        assert!(Solution::is_balanced(Some(Rc::new(RefCell::new(
            TreeNode::new(3)
        )))));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        assert_eq!(false, Solution::is_balanced(root));
    }
}

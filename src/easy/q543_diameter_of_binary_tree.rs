/*
 * @lc app=leetcode id=543 lang=rust
 *
 * [543] Diameter of Binary Tree
 */
struct Solution;
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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    let left = max_depth(node.left.clone());
                    let right = max_depth(node.right.clone());
                    std::cmp::max(left, right) + 1
                }
            }
        }

        fn max_diameter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    let left = max_depth(node.left.clone());
                    let right = max_depth(node.right.clone());
                    std::cmp::max(
                        left + right,
                        std::cmp::max(
                            max_diameter(node.left.clone()),
                            max_diameter(node.right.clone()),
                        ),
                    )
                }
            }
        }

        max_diameter(root)
    }
}
// @lc code=end

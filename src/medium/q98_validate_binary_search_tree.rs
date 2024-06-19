/*
 * @lc app=leetcode id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut array = Vec::new();
        // inorder traverse a valid BST will give a strictly increasing array
        fn inorder(array: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
            match root {
                None => return,
                Some(root) => {
                    let root = root.borrow();
                    inorder(array, root.left.clone());
                    array.push(root.val);
                    inorder(array, root.right.clone());
                }
            }
        }
        inorder(&mut array, root);
        for chunk in array.windows(2) {
            if chunk[0] >= chunk[1] {
                return false;
            }
        }
        true
    }
}
// @lc code=end

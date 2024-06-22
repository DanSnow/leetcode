/*
 * @lc app=leetcode id=105 lang=rust
 *
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
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
    // The first element of preorder is the root of the tree.
    // For all the elements before the root in the inorder, they are in the left subtree.
    // Therefore, we can recursively find the root node in the preorder, remove it, and use the rest of node to build the left subtree
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        type Node = Option<Rc<RefCell<TreeNode>>>;
        assert!(preorder.len() > 0);
        use std::collections::VecDeque;
        let mut preorder = VecDeque::from(preorder);
        fn build_tree(preorder: &mut VecDeque<i32>, inorder: &[i32]) -> Node {
            if preorder.is_empty() || inorder.is_empty() {
                return None;
            }
            let root = preorder[0];
            preorder.pop_front();
            if inorder.is_empty() {
                Some(Rc::new(RefCell::new(TreeNode::new(root))));
            }
            let mut split = inorder.split(|n| n == &root);
            let left = split.next().unwrap();
            let right = split.next().unwrap_or(&[]);
            let left_tree = build_tree(preorder, left);
            let right_tree = build_tree(preorder, right);
            let mut current = TreeNode::new(root);
            current.left = left_tree;
            current.right = right_tree;
            Some(Rc::new(RefCell::new(current)))
        }

        build_tree(&mut preorder, &inorder)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
    }
}

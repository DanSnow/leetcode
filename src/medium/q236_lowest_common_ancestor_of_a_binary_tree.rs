/*
 * @lc app=leetcode id=236 lang=rust
 *
 * [236] Lowest Common Ancestor of a Binary Tree
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
    // ref: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/5000960/python-recursion-and-you-ll-understand-it-or-your-money-back/
    // Question have a important point: both p and q must be in the tree
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // p and q are both in the tree
        if root.is_none() || root == p || root == q {
            return root;
        }

        let root = root.unwrap();
        let root_ref = root.borrow();

        let left = Self::lowest_common_ancestor(root_ref.left.clone(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(root_ref.right.clone(), p, q);

        drop(root_ref);

        match (left, right) {
            (Some(_), Some(_)) => Some(root),
            (Some(left), _) => Some(left),
            (_, Some(right)) => Some(right),
            (None, None) => None,
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    impl TreeNode {
        fn new_node(
            val: i32,
            left: Option<Rc<RefCell<Self>>>,
            right: Option<Rc<RefCell<Self>>>,
        ) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self { val, left, right })))
        }
    }

    #[test]
    fn it_works() {
        let left = TreeNode::new_node(
            5,
            TreeNode::new_node(6, None, None),
            TreeNode::new_node(
                2,
                TreeNode::new_node(7, None, None),
                TreeNode::new_node(4, None, None),
            ),
        );
        let right = TreeNode::new_node(
            1,
            TreeNode::new_node(0, None, None),
            TreeNode::new_node(8, None, None),
        );
        let tree = TreeNode::new_node(3, left.clone(), right.clone());
        assert_eq!(
            3,
            Solution::lowest_common_ancestor(tree, left, right)
                .unwrap()
                .borrow()
                .val
        );
    }
}

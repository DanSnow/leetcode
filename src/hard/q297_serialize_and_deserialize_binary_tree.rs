/*
 * @lc app=leetcode id=297 lang=rust
 *
 * [297] Serialize and Deserialize Binary Tree
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
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Codec;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self::default()
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "".to_owned();
        }

        let mut res = vec![];
        let mut queue = VecDeque::from(vec![root]);

        while let Some(node) = queue.pop_front() {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    res.push(node.val.to_string());
                    queue.push_back(node.left.clone());
                    queue.push_back(node.right.clone());
                }
                None => {
                    res.push("x".to_owned());
                }
            }
        }

        let last_item = res.iter().rev().position(|item| item != "x");

        res.truncate(res.len() - last_item.unwrap());

        res.join(" ")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let mut iter = data.split_ascii_whitespace();
        let mut queue = VecDeque::new();

        let val = iter.next().unwrap().parse::<i32>().unwrap();
        let root = Rc::new(RefCell::new(TreeNode::new(val)));
        queue.push_back(root.clone());

        while let Some(left) = iter.next() {
            let current = queue.pop_front().unwrap();
            let mut current = current.borrow_mut();
            let left_node = if left == "x" {
                None
            } else {
                let val = left.parse::<i32>().unwrap();
                Some(Rc::new(RefCell::new(TreeNode::new(val))))
            };

            if let Some(left_node) = left_node {
                current.left = Some(left_node.clone());
                queue.push_back(left_node);
            }
            let right = iter.next();

            if let Some(right) = right {
                let right_node = if right == "x" {
                    None
                } else {
                    let val = right.parse::<i32>().unwrap();
                    Some(Rc::new(RefCell::new(TreeNode::new(val))))
                };

                if let Some(right_node) = right_node {
                    current.right = Some(right_node.clone());
                    queue.push_back(right_node);
                }
            }
        }

        Some(root)
    }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
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
        let tree = TreeNode::new_node(
            1,
            TreeNode::new_node(2, None, None),
            TreeNode::new_node(
                3,
                TreeNode::new_node(4, None, None),
                TreeNode::new_node(5, None, None),
            ),
        );

        let codec = Codec::new();
        let data = codec.serialize(tree.clone());
        assert_eq!("1 2 3 x x 4 5", data);

        let tree2 = codec.deserialize(data);
        assert_eq!(tree, tree2);
    }
}

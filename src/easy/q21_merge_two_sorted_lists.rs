/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn merge(mut node1: Box<ListNode>, mut node2: Box<ListNode>) -> Option<Box<ListNode>> {
            if node1.val > node2.val {
                let rest_node = node2.next;
                match rest_node {
                    Some(rest_node) => {
                        node2.next = merge(node1, rest_node);
                    }
                    None => {
                        node2.next = Some(node1);
                    }
                }
                Some(node2)
            } else {
                let rest_node = node1.next;
                match rest_node {
                    Some(rest_node) => {
                        node1.next = merge(node2, rest_node);
                    }
                    None => {
                        node1.next = Some(node2);
                    }
                }
                Some(node1)
            }
        }

        match (list1, list2) {
            (Some(node1), Some(node2)) => merge(node1, node2),
            (None, list2) => list2,
            (list1, None) => list1,
        }
    }
}
// @lc code=end

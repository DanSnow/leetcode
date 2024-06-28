/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::{convert::identity, mem, ptr};
        type Node = Option<Box<ListNode>>;
        // it's just a head node to collect sorted list
        let head = Box::new(ListNode::new(-1));
        let mut lists = lists.into_iter().flat_map(identity).collect::<Vec<_>>();

        fn merge_list(mut head: Box<ListNode>, lists: &mut Vec<Box<ListNode>>) -> Node {
            if lists.is_empty() {
                return Some(head);
            }
            let smallest_node = &mut lists[0];
            let mut smallest_val = smallest_node.val;
            let mut smallest_node_index = 0;
            // we need to bypass borrow checker under the following for loop
            let mut smallest_node = smallest_node as *mut Box<ListNode>;
            for (index, node) in lists.iter_mut().enumerate() {
                if node.val < smallest_val {
                    smallest_node = node as *mut _;
                    smallest_val = node.val;
                    smallest_node_index = index;
                }
            }
            // Safety: we know this is from the list
            let smallest_node = unsafe { smallest_node.as_mut().unwrap() };
            let next_node = mem::take(&mut smallest_node.next);
            match next_node {
                Some(node) => {
                    let smallest_node = mem::replace(smallest_node, node);
                    head.next = merge_list(smallest_node, lists);
                    Some(head)
                }
                None => {
                    let node = lists.swap_remove(smallest_node_index);
                    head.next = merge_list(node, lists);
                    Some(head)
                }
            }
        }

        let head = merge_list(head, &mut lists);

        match head {
            Some(head) => head.next,
            None => None,
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    type Node = Option<Box<ListNode>>;
    impl ListNode {
        fn new_node(val: i32, next: Node) -> Node {
            Some(Box::new(ListNode { val, next }))
        }
    }

    #[test]
    fn it_work() {
        assert_eq!(
            ListNode::new_node(
                1,
                ListNode::new_node(
                    1,
                    ListNode::new_node(
                        2,
                        ListNode::new_node(
                            3,
                            ListNode::new_node(
                                4,
                                ListNode::new_node(
                                    4,
                                    ListNode::new_node(5, ListNode::new_node(6, None,))
                                )
                            )
                        )
                    )
                ),
            ),
            Solution::merge_k_lists(vec![
                ListNode::new_node(1, ListNode::new_node(4, ListNode::new_node(5, None))),
                ListNode::new_node(1, ListNode::new_node(3, ListNode::new_node(4, None))),
                ListNode::new_node(2, ListNode::new_node(6, None))
            ])
        )
    }
}

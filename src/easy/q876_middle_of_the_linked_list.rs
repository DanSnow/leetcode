/*
 * @lc app=leetcode id=876 lang=rust
 *
 * [876] Middle of the Linked List
 */

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

struct Solution;

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.as_deref();
        let mut slow = fast.clone();
        while fast.is_some() {
            let fast_next = fast.and_then(|node| node.next.as_deref());
            match fast_next {
                Some(fast_next) => {
                    fast = Some(fast_next);
                    slow = slow.and_then(|node| node.next.as_deref());
                }
                None => break,
            }
            assert!(slow.is_some());
            let fast_next = fast.and_then(|node| node.next.as_deref());
            match fast_next {
                Some(fast_next) => {
                    fast = Some(fast_next);
                }
                None => break,
            }
        }
        slow.map(|node| Box::new(node.clone()))
    }
}
// @lc code=end

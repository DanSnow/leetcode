/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let prev = head;
        if prev.is_none() || prev.as_ref().unwrap().next.is_none() {
            return prev;
        }

        let mut prev = prev.unwrap();

        let mut current = prev.next.take();

        while let Some(mut next) = current {
            current = next.next.take();
            next.next = Some(prev);
            prev = next;
        }

        Some(prev)
    }
}
// @lc code=end

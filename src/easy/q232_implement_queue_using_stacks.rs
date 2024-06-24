/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 */

// @lc code=start
use std::collections::VecDeque;
struct MyQueue {
    data: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.data.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        let mut storage = VecDeque::with_capacity(self.data.len());
        while let Some(x) = self.data.pop_back() {
            storage.push_back(x);
        }
        let res = storage.pop_back().unwrap();
        while let Some(x) = storage.pop_back() {
            self.data.push_back(x);
        }
        res
    }

    fn peek(&mut self) -> i32 {
        let mut storage = VecDeque::with_capacity(self.data.len());
        while let Some(x) = self.data.pop_back() {
            storage.push_back(x);
        }
        let res = *storage.back().unwrap();
        while let Some(x) = storage.pop_back() {
            self.data.push_back(x);
        }
        res
    }

    fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

/*
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end

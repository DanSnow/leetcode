/*
 * @lc app=leetcode id=155 lang=rust
 *
 * [155] Min Stack
 */

// @lc code=start
struct MinStack {
    min: i32,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            min: i32::MAX,
            stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.min = self.min.min(val);
        self.stack.push(val);
    }

    fn pop(&mut self) {
        let top = self.stack.pop();
        if let Some(top) = top {
            if top == self.min {
                self.min = *self.stack.iter().min().unwrap_or(&i32::MAX);
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

/*
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
// @lc code=end

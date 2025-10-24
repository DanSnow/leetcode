use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
struct KthLargest {
    k: usize,
    // wrap in Reverse to make it as a min heap
    heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut obj = Self::default();
        obj.k = k as usize;
        for n in nums {
            obj.add(n);
        }
        obj
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() >= self.k {
            if self.heap.peek().unwrap().0 < val {
                self.heap.pop();
                self.heap.push(Reverse(val));
            }
        } else {
            self.heap.push(Reverse(val));
        }
        assert!(self.heap.len() > 0);
        self.heap.peek().unwrap().0
    }
}

/*
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest() {
        let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(4, obj.add(3));
        assert_eq!(5, obj.add(5));
        assert_eq!(5, obj.add(10));
        assert_eq!(8, obj.add(9));
        assert_eq!(8, obj.add(4));
    }
}

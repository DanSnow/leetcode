/*
 * @lc app=leetcode id=295 lang=rust
 *
 * [295] Find Median from Data Stream
 */

// @lc code=start
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default, Clone, Debug)]
struct MedianFinder {
    is_even: bool,
    small_part: BinaryHeap<i32>,
    large_part: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            is_even: true,
            ..Default::default()
        }
    }

    fn add_num(&mut self, num: i32) {
        self.is_even = !self.is_even;
        self.small_part.push(num);

        if self.is_even {
            let n = self.small_part.pop().unwrap();
            self.large_part.push(Reverse(n));
        } else {
            // balance heap
            loop {
                match (
                    self.small_part.peek().copied(),
                    self.large_part.peek().copied(),
                ) {
                    (Some(small), Some(Reverse(large))) if small > large => {
                        let n = self.large_part.pop().unwrap().0;
                        self.small_part.push(n);
                    }
                    _ => break,
                }
            }
            while self.small_part.len() > self.large_part.len()
                && self.small_part.len() - self.large_part.len() != 1
            {
                let n = self.small_part.pop().unwrap();
                self.large_part.push(Reverse(n));
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.is_even {
            (f64::from(self.small_part.peek().copied().unwrap())
                + f64::from(self.large_part.peek().copied().unwrap().0))
                / 2.0
        } else {
            f64::from(self.small_part.peek().copied().unwrap())
        }
    }
}

/*
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        let mut finder = MedianFinder::new();
        finder.add_num(1);
        finder.add_num(2);
        assert_eq!(1.5, finder.find_median());
        dbg!(&finder);
        finder.add_num(3);
        dbg!(&finder);
        assert_eq!(2.0, finder.find_median());

        let mut finder = MedianFinder::new();
        finder.add_num(1);
        assert_eq!(1.0, finder.find_median());
        let mut finder = MedianFinder::new();
        finder.add_num(1);
        finder.add_num(3);
        finder.add_num(2);
        finder.add_num(3);
        assert_eq!(2.5, finder.find_median());
    }
}

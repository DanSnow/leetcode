/*
 * @lc app=leetcode id=146 lang=rust
 *
 * [146] LRU Cache
 */

// @lc code=start
use std::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    collections::{HashMap, VecDeque},
};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
struct LRUItem {
    key: i32,
    val: i32,
}

#[derive(Debug, Clone)]
struct LRUCache {
    capacity: i32,
    map: HashMap<i32, LRUItem>,
    list: VecDeque<LRUItem>,
}
fn move_to_front(list: &mut VecDeque<LRUItem>, item: LRUItem) {
    list.retain(|x| x.key != item.key);
    list.push_front(item);
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            map: HashMap::with_capacity(capacity as usize + 1),
            list: VecDeque::with_capacity(capacity as usize + 1),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(item) = self.map.get_mut(&key) {
            move_to_front(&mut self.list, *item);
            item.val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(item) = self.map.get_mut(&key) {
            item.val = value;
            move_to_front(&mut self.list, *item);
        } else {
            if self.map.len() >= self.capacity as usize {
                // evict the least recently used item
                let last_item = self.list.pop_back().unwrap();
                self.map.remove(&last_item.key);
            }
            let item = LRUItem { key, val: value };
            self.map.insert(key, item);
            move_to_front(&mut self.list, item);
        }
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end

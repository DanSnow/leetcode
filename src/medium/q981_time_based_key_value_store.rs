/*
 * @lc app=leetcode id=981 lang=rust
 *
 * [981] Time Based Key-Value Store
 */

// @lc code=start

use std::collections::{BTreeMap, HashMap};

struct TimeMap {
    map: HashMap<String, BTreeMap<i32, String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map
            .entry(key)
            .and_modify(|map| {
                map.insert(timestamp, value.clone());
            })
            .or_insert_with(|| {
                let mut map = BTreeMap::new();
                map.insert(timestamp, value);
                map
            });
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map.get(&key).map_or_else(String::new, |map| {
            map.range(..=timestamp)
                .last()
                .map(|(_, s)| s.clone())
                .unwrap_or_else(String::new)
        })
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        let mut obj = TimeMap::new();
        obj.set("foo".to_owned(), "bar".to_owned(), 1);
        assert_eq!("bar", obj.get("foo".to_owned(), 1));
        assert_eq!("bar", obj.get("foo".to_owned(), 3));
        obj.set("foo".to_owned(), "bar2".to_owned(), 4);
        assert_eq!("bar2", obj.get("foo".to_owned(), 4));
        assert_eq!("bar2", obj.get("foo".to_owned(), 5));
    }
}

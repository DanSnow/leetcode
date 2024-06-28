/*
 * @lc app=leetcode id=208 lang=rust
 *
 * [208] Implement Trie (Prefix Tree)
 */

// @lc code=start
use std::{cmp::min, collections::HashMap, mem};

#[derive(Debug, Clone, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_leaf: bool,
}

#[derive(Debug, Clone)]
struct Trie {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            root: Default::default(),
        }
    }

    fn insert(&mut self, word: String) {
        let node = word.chars().fold(&mut self.root, |node, c| {
            node.children.entry(c).or_default()
        });
        node.is_leaf = true;
    }

    fn search_node(&self, word: String) -> Option<&TrieNode> {
        word.chars().fold(Some(&self.root), |node, c| {
            node.and_then(|node| node.children.get(&c))
        })
    }

    fn search(&self, word: String) -> bool {
        let node = self.search_node(word);
        node.map(|node| node.is_leaf).unwrap_or(false)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.search_node(prefix).is_some()
    }
}

/*
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        let mut trie = Trie::new();
        trie.insert("abc".to_owned());
        assert_eq!(true, trie.search("abc".to_owned()));
        assert_eq!(false, trie.search("ab".to_owned()));
        trie.insert("ab".to_owned());
        assert_eq!(true, trie.search("ab".to_owned()));
        trie.insert("ab".to_owned());
        assert_eq!(true, trie.search("ab".to_owned()));
        dbg!(&trie);
        assert_eq!(true, trie.starts_with("abc".to_owned()));
    }
}

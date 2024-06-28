/*
 * @lc app=leetcode id=208 lang=rust
 *
 * [208] Implement Trie (Prefix Tree)
 */

// @lc code=start
// FIXME: has error in large test case
use std::{cmp::min, mem};

#[derive(Debug, Clone)]
enum TrieNode {
    Prefix {
        prefix: String,
        children: Vec<TrieNode>,
    },
    Leaf {
        text: String,
    },
}

impl TrieNode {
    fn insert(&mut self, text: String, view: &str) {
        match self {
            Self::Prefix { prefix, children } => {
                let mut is_inserted = false;
                for node in children.iter_mut() {
                    match node {
                        Self::Prefix { prefix, .. } => {
                            if view.starts_with(prefix.as_str()) {
                                let view = &view[prefix.len()..];
                                node.insert(text.clone(), view);
                                is_inserted = true;
                                break;
                            }
                        }
                        Self::Leaf { text: t } => {
                            if t == text.as_str() {
                                is_inserted = true;
                                break;
                            }
                            let common_prefix = get_common_prefix(view, t.as_str());
                            if !common_prefix.is_empty() {
                                let prefix_node = Self::Prefix {
                                    prefix: common_prefix.to_owned(),
                                    children: vec![Self::Leaf { text: text.clone() }],
                                };
                                let leaf = mem::replace(node, prefix_node);
                                let Self::Prefix {
                                    ref mut children, ..
                                } = node
                                else {
                                    panic!("impossible");
                                };
                                children.push(leaf);
                                is_inserted = true;
                                break;
                            }
                        }
                    }
                }
                if !is_inserted {
                    children.push(Self::Leaf { text })
                }
            }
            Self::Leaf { .. } => panic!("Attempt insert into leaf node"),
        }
    }

    fn search(&self, text: &str, view: &str) -> bool {
        match self {
            Self::Prefix { children, .. } => {
                for node in children.iter() {
                    let res = match node {
                        Self::Prefix { prefix, children } => {
                            if view.starts_with(prefix.as_str()) {
                                let view = &view[prefix.len()..];
                                for node in children.iter() {
                                    if node.search(text, view) {
                                        return true;
                                    }
                                }
                            }
                            false
                        }
                        Self::Leaf { text: t } => t == text,
                    };
                    if res {
                        return true;
                    }
                }
                false
            }
            Self::Leaf { text: t } => t == text,
        }
    }

    fn starts_with(&self, text: &str, view: &str) -> bool {
        match self {
            Self::Prefix { children, .. } => {
                for node in children.iter() {
                    let res = match node {
                        Self::Prefix { prefix, children } => {
                            if view.starts_with(prefix.as_str()) {
                                let view = &view[prefix.len()..];
                                for node in children.iter() {
                                    if node.starts_with(text, view) {
                                        return true;
                                    }
                                }
                            }
                            false
                        }
                        Self::Leaf { text: t } => t.starts_with(text),
                    };
                    if res {
                        return true;
                    }
                }
                false
            }
            Self::Leaf { text: t } => t.starts_with(text),
        }
    }
}

fn get_common_prefix<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.is_empty() || right.is_empty() {
        return "";
    }

    // Safety: in the constraints, there is only letters
    let l = left.as_bytes();
    let r = right.as_bytes();

    if l[0] != r[0] {
        return "";
    }

    let len = min(l.len(), r.len());
    for i in 1..len {
        if l[i] != r[i] {
            return &left[0..(i - 1)];
        }
    }
    &left[0..len]
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
            root: TrieNode::Prefix {
                prefix: String::new(),
                children: vec![],
            },
        }
    }

    fn insert(&mut self, word: String) {
        self.root.insert(word.clone(), &word);
    }

    fn search(&self, word: String) -> bool {
        self.root.search(&word, &word)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.root.starts_with(&prefix, &prefix)
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

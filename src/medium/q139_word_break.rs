/*
 * @lc app=leetcode id=139 lang=rust
 *
 * [139] Word Break
 */

struct Solution;

// @lc code=start

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    index: usize,
    is_leaf: bool,
}

#[derive(Debug, Clone)]
struct Trie {
    root: TrieNode,
}

#[derive(Debug, Clone, Copy)]
struct PossibleItem<'a> {
    index: usize,
    remaining: &'a str,
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

    fn insert(&mut self, word: String, index: usize) {
        let node = word.chars().fold(&mut self.root, |node, c| {
            node.children.entry(c).or_default()
        });
        node.is_leaf = true;
        node.index = index;
    }

    fn get_possibles<'a>(&self, text: &'a str) -> Vec<PossibleItem<'a>> {
        let mut current_node = &self.root;
        let mut possible = vec![];
        for (index, c) in text.chars().enumerate() {
            current_node = match current_node.children.get(&c) {
                Some(node) => {
                    if node.is_leaf {
                        possible.push(PossibleItem {
                            remaining: &text[(index + 1)..],
                            index: node.index,
                        })
                    }
                    node
                }
                None => return possible,
            };
        }
        possible
    }
}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut trie = Trie::new();
        let mut memo = HashMap::new();

        for (index, word) in word_dict.into_iter().enumerate() {
            trie.insert(word, index);
        }

        fn dfs<'a>(s: &'a str, trie: &Trie, memo: &mut HashMap<&'a str, bool>) -> bool {
            let possibles = trie.get_possibles(s);
            for PossibleItem { remaining, .. } in possibles {
                if remaining.is_empty() {
                    return true;
                }
                match memo.get(remaining) {
                    Some(&true) => return true,
                    Some(&false) => (),
                    None => {
                        let ans = dfs(remaining, trie, memo);
                        memo.insert(remaining, ans);
                        if ans {
                            return true;
                        }
                    }
                }
            }
            false
        }

        dfs(&s, &trie, &mut memo)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            true,
            Solution::word_break(
                "leetcode".to_owned(),
                vec!["leet".to_owned(), "code".to_owned()]
            )
        );
    }
}

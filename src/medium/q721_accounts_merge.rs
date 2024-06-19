/*
 * @lc app=leetcode id=721 lang=rust
 *
 * [721] Accounts Merge
 */

struct Solution;

// @lc code=start
impl Solution {
    // DisjointSet Union
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        use std::{cmp::Ordering, collections::HashMap, iter};

        #[derive(Debug, Clone)]
        struct DisjointSet {
            parent: Vec<usize>,
            rank: Vec<u32>,
        }

        impl DisjointSet {
            fn new(size: usize) -> Self {
                Self {
                    parent: (0..size).collect(),
                    rank: vec![0; size],
                }
            }

            fn find_parent(&self, node: usize) -> usize {
                if self.parent[node] != node {
                    self.find_parent(self.parent[node])
                } else {
                    node
                }
            }

            fn normalize_parent(&mut self) {
                for i in 0..self.parent.len() {
                    if self.parent[i] != i {
                        // ensure every node has a root node as parent
                        self.parent[i] = self.find_parent(i)
                    }
                }
            }

            fn list_children<'a>(&'a self, node: usize) -> impl Iterator<Item = usize> + 'a {
                let mut index = 0;
                iter::from_fn(move || {
                    while index < self.parent.len() && self.parent[index] != node && index != node {
                        index += 1;
                    }
                    if index < self.parent.len() {
                        index += 1;
                        Some(index - 1)
                    } else {
                        None
                    }
                })
            }

            fn uni(&mut self, node1: usize, node2: usize) {
                let node1 = self.find_parent(node1);
                let node2 = self.find_parent(node2);

                match self.rank[node1].cmp(&self.rank[node2]) {
                    Ordering::Equal => {
                        self.parent[node2] = node1;
                        self.rank[node1] += 1;
                    }
                    Ordering::Greater => {
                        self.parent[node1] = node2;
                    }
                    Ordering::Less => {
                        self.parent[node2] = node1;
                    }
                }
            }
        }

        let mut ds = DisjointSet::new(accounts.len());
        let mut emails = HashMap::new();

        for (i, account) in accounts.iter().enumerate() {
            let [_, e @ ..] = account.as_slice() else {
                continue;
            };

            for email in e {
                if emails.contains_key(email) {
                    ds.uni(emails[email], i);
                } else {
                    emails.insert(email, i);
                }
            }
        }

        let mut res = vec![];

        ds.normalize_parent();

        for i in 0..accounts.len() {
            if ds.find_parent(i) == i {
                let mut account = vec![accounts[i][0].clone()];
                for email in &accounts[i][1..] {
                    if !account.contains(email) {
                        // Data may already contain duplicate email
                        account.push(email.to_owned());
                    }
                }
                for child in ds.list_children(i) {
                    for email in &accounts[child][1..] {
                        if !account.contains(&email) {
                            account.push(email.to_owned());
                        }
                    }
                }
                account[1..].sort_unstable();
                res.push(account);
            }
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        Solution::accounts_merge(vec![
            vec![
                "John".to_owned(),
                "johnsmith@mail.com".to_owned(),
                "john_newyork@mail.com".to_owned(),
            ],
            vec![
                "John".to_owned(),
                "johnsmith@mail.com".to_owned(),
                "john00@mail.com".to_owned(),
            ],
            vec!["Mary".to_owned(), "mary@mail.com".to_owned()],
            vec!["John".to_owned(), "johnnybravo@mail.com".to_owned()],
        ]);
    }
}

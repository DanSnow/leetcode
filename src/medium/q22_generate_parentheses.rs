/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        use std::collections::HashSet;
        let mut res = vec![];
        let mut seen = HashSet::new();
        fn generate(
            res: &mut Vec<String>,
            seen: &mut HashSet<String>,
            mut path: String,
            depth: i32,
            closing: i32,
        ) {
            if depth == 0 {
                for _ in 0..closing {
                    path.push(')');
                }
                if !path.is_empty() {
                    if !seen.contains(&path) {
                        seen.insert(path.clone());
                        res.push(path);
                    }
                }
                return;
            }

            if depth > 1 {
                let mut path = path.clone();
                // defer closing
                path.push('(');
                generate(res, seen, path, depth - 1, closing + 1);
            }

            if closing > 1 {
                // flush closing
                for i in 1..=closing {
                    let mut path = path.clone();
                    for _ in 0..i {
                        path.push(')');
                    }
                    generate(res, seen, path, depth, closing - i)
                }
            }

            let mut path_immediate = path;
            // immediate closing
            path_immediate.push_str("()");
            generate(res, seen, path_immediate, depth - 1, closing);
        }
        generate(&mut res, &mut seen, String::new(), n, 0);
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            vec![
                "(()())".to_owned(),
                "(())()".to_owned(),
                "((()))".to_owned(),
                "()(())".to_owned(),
                "()()()".to_owned()
            ],
            Solution::generate_parenthesis(3)
        );
    }
}

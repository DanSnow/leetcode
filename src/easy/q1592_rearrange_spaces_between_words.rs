struct Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        if text.is_empty() {
            return String::new();
        }
        let mut word_start = None;
        let mut spaces = 0;
        let mut words = vec![];
        for (i, c) in text.char_indices() {
            if c.is_whitespace() {
                if let Some(start) = word_start.take() {
                    words.push(&text[start..i]);
                }
                spaces += 1;
            } else if word_start.is_none() {
                word_start = Some(i);
            }
        }

        if let Some(start) = word_start {
            words.push(&text[start..text.len()]);
        }

        if spaces == 0 {
            return text;
        }

        let inner_count = words.len() - 1;

        if inner_count == 0 {
            return format!("{}{:spaces$}", words[0], "");
        }

        let between = spaces / inner_count;
        let after = spaces % inner_count;

        format!("{}{:after$}", words.join(&" ".repeat(between)), "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorder_spaces() {
        assert_eq!(
            "this   is   a   sentence",
            Solution::reorder_spaces("  this   is  a sentence ".to_owned())
        );

        assert_eq!(
            "practice   makes   perfect ",
            Solution::reorder_spaces(" practice   makes   perfect".to_owned())
        );

        assert_eq!("a", Solution::reorder_spaces("a".to_owned()))
    }
}

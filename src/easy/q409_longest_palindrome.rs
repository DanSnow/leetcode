pub fn longest_palindrome(s: String) -> i32 {
    use std::collections::HashMap;

    let mut count = HashMap::new();

    for c in s.chars() {
        count
            .entry(c)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
    }

    let mut odd = 0;
    let even_count = count
        .values()
        .copied()
        .map(|n| {
            match n {
                ..0 => 0,
                0 => 0,
                // even
                1.. if n % 2 == 0 => n / 2,
                // odd
                1.. if n % 2 == 1 => {
                    odd = 1;
                    (n - 1) / 2
                }
                _ => unreachable!(),
            }
        })
        .sum::<i32>();

    even_count * 2 + odd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(7, longest_palindrome("abccccdd".to_owned()));
        assert_eq!(3, longest_palindrome("ccc".to_owned()));
        assert_eq!(1, longest_palindrome("a".to_owned()));
        assert_eq!(0, longest_palindrome("".to_owned()));
    }
}

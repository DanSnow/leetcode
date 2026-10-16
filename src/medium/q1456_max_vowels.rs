fn max_vowels(s: String, k: i32) -> i32 {
    use std::cmp;
    let k = k as usize;
    let s = s.as_bytes();
    let mut ans = 0;

    for i in 0..(s.len() - k + 1) {
        let chunk = &s[i..(i + k)];
        let count = chunk
            .iter()
            .copied()
            .filter(|c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u'))
            .count();
        ans = cmp::max(count as i32, ans);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_vowels() {
        assert_eq!(max_vowels("abciiidef".into(), 3), 3);
        assert_eq!(max_vowels("aeiou".into(), 2), 2);
        assert_eq!(max_vowels("leetcode".into(), 3), 2);
        assert_eq!(max_vowels("weallloveyou".into(), 7), 4);
    }
}
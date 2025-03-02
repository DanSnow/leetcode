pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    use std::{cmp, collections::HashMap};

    fn digit_sum(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }

    #[derive(Debug, Copy, Clone)]
    struct Num {
        num: i32,
        sum: i32,
    }

    impl Num {
        fn new(num: i32) -> Self {
            Self {
                num,
                sum: digit_sum(num),
            }
        }
    }

    let mut max = -1;

    let mut map = HashMap::new();

    for num in nums.iter().copied() {
        let num = Num::new(num);
        map.entry(num.sum)
            .and_modify(|origin: &mut Num| {
                max = cmp::max(max, origin.num + num.num);
                if origin.num < num.num {
                    *origin = num;
                }
            })
            .or_insert(num);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::maximum_sum;

    #[test]
    fn test_maximum_sum() {
        assert_eq!(54, maximum_sum(vec![18, 43, 36, 13, 7]));
    }
}

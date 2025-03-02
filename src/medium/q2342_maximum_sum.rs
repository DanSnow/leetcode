pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    use std::cmp;

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

    // every num is less than 10^9, so the number with max sum will be `99999999` => 81
    let mut map = vec![-1; 82];

    for num in nums.iter().copied() {
        let num = Num::new(num);
        let index = num.sum as usize;
        if map[index] != -1 {
            max = cmp::max(max, map[index] + num.num);
        }
        map[index] = cmp::max(map[index], num.num);
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

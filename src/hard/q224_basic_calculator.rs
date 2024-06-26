/*
 * @lc app=leetcode id=224 lang=rust
 *
 * [224] Basic Calculator
 */

struct Solution;

// @lc code=start

use std::mem;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Consume,
    Number,
    Operator,
}

#[derive(Debug, Clone)]
struct Calculator<'a> {
    chars: &'a [u8],
    index: usize,
    state: State,
    operators: Vec<u8>,
    nums: Vec<i32>,
    sign: i32,
    num: i32,
}

impl<'a> Calculator<'a> {
    fn new(chars: &'a [u8]) -> Self {
        Self {
            chars,
            index: 0,
            state: State::Consume,
            operators: vec![],
            nums: vec![],
            sign: 1,
            num: 0,
        }
    }

    fn next(&mut self) -> Option<u8> {
        let next = self.chars.get(self.index).copied();
        self.index += 1;
        next
    }

    fn start_number(&mut self, n: u8) {
        assert!(n.is_ascii_digit());

        self.num = 0;
        self.num += (n - b'0') as i32;
        self.state = State::Number;
    }

    fn add_digit(&mut self, n: u8) {
        assert!(n.is_ascii_digit());

        self.num *= 10;
        self.num += (n - b'0') as i32;
        self.state = State::Number;
    }

    fn start_negative(&mut self, n: u8) {
        assert_eq!(b'-', n);
        self.sign = -1;
        self.state = State::Number;
    }

    fn eval_op(&mut self, op: u8) {
        let right = self.nums.pop().unwrap();
        let left = self.nums.pop().unwrap();

        let ans = match op {
            b'+' => left + right,
            b'-' => left - right,
            b'*' => left * right,
            b'/' => left / right,
            _ => unreachable!(),
        };
        self.nums.push(ans);
    }

    fn eval_param(&mut self) {
        let mut nums = vec![];
        let mut operators = vec![];

        while let Some(op) = self.operators.pop() {
            if op == b'(' {
                break;
            }
            let right = self.nums.pop().unwrap();
            let left = self.nums.pop().unwrap();
            operators.push(op);
            nums.push(right);
            nums.push(left);
        }

        let mut nums = nums.into_iter();

        let mut result = 0;

        for op in operators {
            let left = nums.next().unwrap();
            let right = nums.next().unwrap();
            result += match op {
                b'+' => left + right,
                b'-' => left - right,
                _ => unreachable!(),
            };
        }

        self.nums.push(result);
    }

    fn eval_all(&mut self) {
        let nums = mem::take(&mut self.nums);
        let mut nums = nums.into_iter();
        let operators = mem::take(&mut self.operators);

        let mut result = nums.next().unwrap();

        for op in operators {
            let right = nums.next().unwrap();
            result = match op {
                b'+' => result + right,
                b'-' => result - right,
                _ => unreachable!(),
            };
        }

        self.nums.push(result);
    }

    fn push_op(&mut self, op: u8) {
        assert!(matches!(op, b'+' | b'-' | b'*' | b'/'));
        self.sign = 1;
        self.operators.push(op);
        self.state = State::Operator;
    }

    fn push_param(&mut self, op: u8) {
        assert_eq!(b'(', op);
        self.state = State::Consume;
        self.operators.push(op);
    }

    fn consume_token(&mut self, c: u8) {
        match c {
            b'(' => self.push_param(c),
            b'+' | b'-' | b'*' | b'/' => {
                self.push_op(c);
            }
            b')' => {
                self.eval_param();
            }
            n if c.is_ascii_digit() => {
                self.start_number(n);
            }
            _ => (),
        }
    }

    fn finish_number(&mut self) {
        self.nums.push(self.num * self.sign);
        self.num = 0;
        self.sign = 1;
    }
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let mut calculator = Calculator::new(s);

        while let Some(c) = calculator.next() {
            match calculator.state {
                State::Consume => calculator.consume_token(c),
                State::Operator => match c {
                    b'(' => {
                        calculator.push_param(c);
                    }
                    b'-' => {
                        calculator.start_negative(c);
                    }
                    b' ' => (),
                    n if c.is_ascii_digit() => {
                        calculator.start_number(n);
                    }
                    _ => unreachable!(),
                },
                // TODO: handle token
                State::Number => {
                    if c.is_ascii_digit() {
                        calculator.add_digit(c);
                    } else {
                        calculator.finish_number();
                        loop {
                            match calculator.operators.last() {
                                Some(b'*' | b'/') => {
                                    let op = calculator.operators.pop().unwrap();
                                    calculator.eval_op(op);
                                }
                                _ => break,
                            }
                        }
                        calculator.state = State::Consume;
                        calculator.consume_token(c);
                    }
                }
            }
        }

        if calculator.state == State::Number {
            calculator.finish_number();
        }
        calculator.eval_all();
        calculator.nums[0]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(3, Solution::calculate("1 + 2".to_owned()));
        assert_eq!(-1, Solution::calculate("1 + -2".to_owned()));
        // assert_eq!(5, Solution::calculate("1 + ( 1 +1) * 2".to_owned()));
        assert_eq!(-1, Solution::calculate(" 2-1 + 2 ".to_owned()));
    }
}

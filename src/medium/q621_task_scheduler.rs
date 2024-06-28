/*
 * @lc app=leetcode id=621 lang=rust
 *
 * [621] Task Scheduler
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        use std::collections::HashMap;

        if tasks.is_empty() {
            return 0;
        }
        let mut cycle = 0;
        let mut map = HashMap::new();
        let mut cool_down_until = tasks
            .iter()
            .copied()
            .map(|c| (c, 0))
            .collect::<HashMap<char, i32>>();
        tasks.iter().copied().fold(&mut map, |map, c| {
            map.entry(c)
                .and_modify(|count: &mut i32| {
                    *count += 1;
                })
                .or_insert(1);
            map
        });

        while !map.is_empty() {
            let mut max_frequency = 0;
            let mut max_task = None;
            // find first available task with maximum appear frequency
            for (&task, &frequency) in map.iter() {
                if frequency > max_frequency && *cool_down_until.get(&task).unwrap() <= cycle {
                    max_task = Some(task);
                    max_frequency = frequency;
                }
            }
            if let Some(task) = max_task {
                cool_down_until.insert(task, cycle + n + 1);
                let frequency = max_frequency - 1;
                if frequency == 0 {
                    map.remove(&task);
                } else {
                    map.insert(task, frequency);
                }
            }
            cycle += 1;
        }

        cycle
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            8,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2)
        );
    }
}

/*
 * @lc app=leetcode id=1235 lang=rust
 *
 * [1235] Maximum Profit in Job Scheduling
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        use std::cmp::max;

        if start_time.len() == 0 {
            return 0;
        }

        assert!(start_time.len() == end_time.len() && end_time.len() == profit.len());

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
        struct Job {
            start_time: i32,
            end_time: i32,
            profit: i32,
        }

        let mut jobs = vec![];

        for i in 0..start_time.len() {
            let job = Job {
                start_time: start_time[i],
                end_time: end_time[i],
                profit: profit[i],
            };
            jobs.push(job);
        }

        jobs.sort_unstable_by(|left, right| {
            left.end_time
                .cmp(&right.end_time)
                .then(left.start_time.cmp(&right.start_time))
        });

        let mut scheduling_profits = vec![0; jobs.len()];
        let mut max_profit = 0;

        for (index, job) in jobs.iter().copied().enumerate() {
            let start_index = jobs.binary_search_by_key(&job.start_time, |job| job.end_time);
            let start_profit = match start_index {
                Ok(start_index) => scheduling_profits[start_index],
                Err(start_bound_index) => {
                    if start_bound_index == 0 {
                        0
                    } else {
                        // when not found, the index is the right bound
                        scheduling_profits[start_bound_index - 1]
                    }
                }
            };
            // add previous no conflict job profit
            let mut current_profit = job.profit + start_profit;
            // check if we have better profit from previous jobs
            // if we do this to all the jobs, we can make sure to have the best profit
            if index > 0 {
                current_profit = max(current_profit, scheduling_profits[index - 1]);
            }
            scheduling_profits[index] = current_profit;
            max_profit = max(current_profit, max_profit);
        }

        max_profit
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(
            120,
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70])
        );
        assert_eq!(
            150,
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            )
        );

        assert_eq!(
            41,
            Solution::job_scheduling(
                vec![6, 15, 7, 11, 1, 3, 16, 2],
                vec![19, 18, 19, 16, 10, 8, 19, 8],
                vec![2, 9, 1, 19, 5, 7, 3, 19]
            )
        );

        assert_eq!(
            120,
            Solution::job_scheduling(
                vec![1, 2, 3, 3],
                vec![3, 4, 5, 1000000000],
                vec![50, 10, 40, 70]
            )
        )
    }
}

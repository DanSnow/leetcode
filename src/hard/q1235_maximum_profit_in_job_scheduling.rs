/*
 * @lc app=leetcode id=1235 lang=rust
 *
 * [1235] Maximum Profit in Job Scheduling
 */

struct Solution;

// @lc code=start
impl Solution {
    // FIXME: runtime error
    pub fn job_scheduling(mut start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        use std::cmp::max;

        if start_time.len() == 0 {
            return 0;
        }

        assert!(start_time.len() == end_time.len() && end_time.len() == profit.len());

        let max_end = end_time.iter().copied().max().unwrap();

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

        jobs.sort_unstable_by_key(|job| job.start_time);
        start_time.sort_unstable();

        // TODO: reduce space usage
        let mut scheduling_profits = vec![0; (max_end + 1) as usize];
        let mut max_profit = 0;

        for job in jobs.iter().copied() {
            scheduling_profits[job.end_time as usize] = max(
                // add previous job profit
                job.profit + scheduling_profits[job.start_time as usize],
                scheduling_profits[job.end_time as usize],
            );
            let current_profit = scheduling_profits[job.end_time as usize];
            let update_points = start_time.partition_point(|time| *time < job.end_time);
            for i in start_time[update_points..].iter().copied() {
                if scheduling_profits[i as usize] <= current_profit {
                    scheduling_profits[i as usize] = current_profit
                } else {
                    break;
                }
            }
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

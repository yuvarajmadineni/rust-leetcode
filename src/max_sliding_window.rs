use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut queue = VecDeque::new();
    let mut left = 0;
    let mut res = vec![];

    for j in 0..nums.len() {
        while !queue.is_empty() && nums[j] > nums[*queue.back().unwrap()] {
            queue.pop_back();
        }
        queue.push_back(j);
        if left > *queue.front().unwrap() {
            queue.pop_front();
        }

        if j + 1 >= k as usize {
            res.push(nums[*queue.front().unwrap()]);
            left += 1;
        }
    }

    res
}

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut dict: HashSet<i32> = HashSet::with_capacity(nums.len());
        !nums.into_iter().all(|num| dict.insert(num))
    }
}
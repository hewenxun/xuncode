use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }

        let mut dict: HashMap<char, i32> = HashMap::with_capacity(26);
        s.chars().for_each(|key| *dict.entry(key).or_insert(0) += 1);
        t.chars().for_each(|key| *dict.entry(key).or_insert(0) -= 1);
        dict.into_iter().all(|(_, val)| val == 0)
    }
}
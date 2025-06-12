pub struct Solution;
/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if count_whitespace(&s) > 0 {
            return 1;
        }
        let mut map: HashMap<&char, i32> = HashMap::new();
        let mut max_len = 0;
        let chars: Vec<char> = s.trim().chars().collect();
        let mut result_array: Vec<i32> = vec![];
        let mut max_array: Vec<usize> = vec![];

        for (index, char) in chars.iter().enumerate() {

            map.insert(char, index.try_into().unwrap());
            println!("Map after insert: {:#?}", map);

            result_array.clear(); // clear before rebuilding
            for (_, value) in &map {
                result_array.push(*value);
            }
            max_len = longest_consecutive(&result_array);
            println!("Longest Consecutive Length: {}", max_len);

            max_array.push(max_len);
            println!("Max Array So Far: {:?}", max_array);
        }

        println!("========================================");
        println!("Final max_array: {:?}", max_array);
        println!("max() of max_array: {:#?}", max(&max_array));
        println!("Final max_len: {:#?}", max_len);

        return max(&max_array).try_into().unwrap_or(1);
    }
}

fn longest_consecutive(nums: &Vec<i32>) -> usize {
    let set: HashSet<_> = nums.iter().cloned().collect();
    let mut longest = 0;

    for &num in &set {
        // Only try to build a sequence from the "start" of one
        if !set.contains(&(num - 1)) {
            let mut current = num;
            let mut streak = 1;

            while set.contains(&(current + 1)) {
                current += 1;
                streak += 1;
            }

            longest = longest.max(streak);
        }
    }

    longest
}

fn max(nums: &Vec<usize>) -> usize {
    let mut max_val = usize::MIN;
    for (_, &val) in nums.iter().enumerate() {
        if val > max_val {
            max_val = val;
        }
    }

    max_val
}

fn count_whitespace(s: &str) -> usize {
    s.chars().filter(|c| c.is_whitespace()).count()
}

// @lc code=end

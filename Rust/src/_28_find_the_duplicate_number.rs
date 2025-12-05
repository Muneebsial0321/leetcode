use std::collections::HashMap;

pub struct Solution;
/*
 * @lc app=leetcode id=287 lang=rust
 *
 * [287] Find the Duplicate Number
 */

// @lc code=start
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for i in 0..nums.len() {
            if let Some(v) = map.get(&nums[i]) {
                map.insert(nums[i], *v + 1);
                return nums[i]
            } else {
                map.insert(nums[i], 1);
            }
            println!("k:{},v:{}", nums[i], map.get(&nums[i]).unwrap());
        }
        -1
    }
}
// @lc code=end

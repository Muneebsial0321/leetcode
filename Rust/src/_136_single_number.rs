pub struct Solution;
/*
 * @lc app=leetcode id=136 lang=rust
 *
 * [136] Single Number
 */

// @lc code=start
impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        nums.sort();
        for i in 0..nums.len() {
            if stack.is_empty() {
                stack.push(nums[i]);
                continue;
            }
            if *stack.last().unwrap() == nums[i] {
                stack.pop();
                continue;
            } else {
                stack.push(nums[i]);
                continue;
            }
        }
        if stack.is_empty() {
            return -1;
        }
        return *stack.last().unwrap();
    }
}
// @lc code=end

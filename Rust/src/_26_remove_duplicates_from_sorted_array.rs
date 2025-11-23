pub struct Solution {}
/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[k-1]{
                nums[k] = nums[i]; 
                k = k + 1;
            }
        }
        k as i32
    }
}
// @lc code=end

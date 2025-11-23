pub struct Solution;
/*
 * @lc app=leetcode id=704 lang=rust
 *
 * [704] Binary Search
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: isize = 0;
        let mut right: isize = nums.len() as isize - 1;
        let mut mid = 0;

        while true {
            mid = (right + left / 2) as usize;

            if left - right == 1 {
                return -1;
            }

            if target == nums[mid] {
                return mid as i32;
            }

            if target < nums[mid] {
                right = mid as isize - 1;
            }

            if target > nums[mid] {
                left = mid as isize + 1;
            }
        }
        mid as i32
    }
}
// @lc code=end

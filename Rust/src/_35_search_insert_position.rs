pub struct Solution;
/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (Some(nums.len()).unwrap_or(0) - 1 ) as i32;
        let mut mid: i32 = 0;

        while true {
            mid = (right + left ) /2;

            if left - right == 1 {
                return left;
            }

            if nums[mid as usize] == target {
                return mid as i32;
            }

            if nums[mid as usize] > target {
                right = mid - 1;
                continue;
            }

            if nums[mid as usize] < target {
                left = mid + 1;
                continue;
            }
        }

        mid as i32
    }
}
// @lc code=end

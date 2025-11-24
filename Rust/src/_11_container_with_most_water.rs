pub struct Solution {}

/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i64 {
        let mut max_water:i64 = 0;
        let mut left_pointer = 0;
        let mut right_pointer = height.len() - 1;
        while left_pointer < right_pointer {
            let width = left_pointer.abs_diff(right_pointer);
            let ht:i64 = std::cmp::min(height[left_pointer] as i64 , height[right_pointer] as i64);
            
            max_water = std::cmp::max(max_water, ht as i64 * width as i64);
            if height[left_pointer] < height[right_pointer] {
                left_pointer = left_pointer + 1;
            } else {
                right_pointer = right_pointer - 1;
            }

        }
        max_water
    }
}
// @lc code=end

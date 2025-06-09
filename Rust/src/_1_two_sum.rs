pub struct Solution;
/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result_array: Vec<i32> = Vec::new();
        let mut i_outer  = 0;
        while i_outer <= nums.len()-1  {
            for (i_inner, e_inner) in nums.iter().enumerate() {
                if i_inner <= i_outer {
                    continue;
                }
                if nums[i_outer] + e_inner == target {
                    result_array.push(i_outer.try_into().unwrap());
                    result_array.push(i_inner.try_into().unwrap());
                    break;
                }
            }
        i_outer +=1;
        }

        return result_array;
    }
}

// @lc code=end

pub struct Solution;
/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Find the Index of the First Occurrence in a String
 */

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(index)=>{
                return index as i32
            }
            None=> return -1
        }
    }
}
// @lc code=end

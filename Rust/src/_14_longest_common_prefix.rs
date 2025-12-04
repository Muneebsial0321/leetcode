pub struct Solution;
/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() || strs[0] == "" {
            return "".to_string();
        }

        let mut base_str = String::from("");

        for (i, char) in strs[0].char_indices() {
            for j in 0..strs.len() {
                if char != strs[j].chars().nth(i).unwrap_or('\0') {
                    return base_str;
                }
            }
            base_str.push_str(&char.to_string());
        }

        base_str
    }
}
// @lc code=end

pub struct Solution;
/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let new_str: Vec<char> = s.trim().chars().collect();

        let mut hash_map: HashMap<char, char> = HashMap::new();
        hash_map.insert('(', ')');
        hash_map.insert('{', '}');
        hash_map.insert('[', ']');

        let mut stack: Vec<char> = Vec::new();

        for i in new_str {
            if i == '(' || i == '{' || i == '[' {
                stack.push(i);
                continue;
            }
            if !stack.is_empty()
                && stack[stack.len() - 1] == '('
                && i == ')'
            {
                stack.pop();
                continue;
            }
            if !stack.is_empty()
                && stack[stack.len() - 1] == '{'
                && i == '}'
            {
                stack.pop();
                continue;
            }
            if !stack.is_empty()
                && stack[stack.len() - 1] == '['
                && i == ']'
            {
                stack.pop();
                continue;
            }
            stack.push(i);
        }

        return stack.is_empty();
    }
}
// @lc code=end

pub struct Solution {
    version: i32,
}

impl Solution {
    pub fn isBadVersion(&self, version: i32) -> bool {
        if version == 5 {
            return true;
        }
        return false;
    }
}
/*
 * @lc app=leetcode id=278 lang=rust
 *
 * [278] First Bad Version
 */

// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut right = n;
        let mut left = 1;
        let mut mid = 0;
// [1,2,3,4,5]
        while left < right {
            mid = left + (right - left) / 2;

            if self.isBadVersion(mid) {
                right = mid;
            }

            if !self.isBadVersion(mid) {
                left = mid + 1
            }
        }
        print!("outher mid {}",mid);
        return left;
    }
}
// @lc code=end

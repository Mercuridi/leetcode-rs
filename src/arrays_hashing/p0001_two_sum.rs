// Leetcode     #0001
// Neetcode 150 #0003

/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.


Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]


Constraints:

    2 <= nums.length <= 104
    -109 <= nums[i] <= 109
    -109 <= target <= 109
    Only one valid answer exists.

 */

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() < 2 {return Vec::new()}

    for i in 0..nums.len() {
        let current = nums[i];
        for j in (i + 1)..nums.len() {
            let other = nums[j];
            if current + other == target {
                return Vec::from([i as i32, j as i32])
            }
        }
    }
    return Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0001() {
        assert_eq!(two_sum(Vec::from([2, 7, 11, 15, 9]), 9), [0, 1]);
        assert_eq!(two_sum(Vec::from([3, 2, 4]), 6), [1, 2]);
        assert_eq!(two_sum(Vec::from([3, 3]), 6), [0, 1]);
    }
}
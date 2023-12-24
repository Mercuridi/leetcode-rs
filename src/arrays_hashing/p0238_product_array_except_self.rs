// Leetcode     #0238
// Neetcode 150 #0006

/*
Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

You must write an algorithm that runs in O(n) time and without using the division operation.

 

Example 1:

Input: nums = [1,2,3,4]
Output: [24,12,8,6]

Example 2:

Input: nums = [-1,1,0,-3,3]
Output: [0,0,9,0,0]

 

Constraints:

    2 <= nums.length <= 105
    -30 <= nums[i] <= 30
    The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

*/

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() < 2 {
        return Vec::new()
    }
    let mut answer: Vec<i32> = vec![1; nums.len()];
    let mut n = 1;
    for i in 1..nums.len() {
        n *= nums[i - 1];
        answer[i] = n;
    }
    n = 1;
    for i in (0..nums.len() - 1).rev() {
        n *= nums[i + 1];
        answer[i] *= n;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0238() {
        assert_eq!(product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
        assert_eq!(product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
    }
}
// Leetcode     #0347
// Neetcode 150 #0005

/*
Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

 

Example 1:

Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]

Example 2:

Input: nums = [1], k = 1
Output: [1]

 

Constraints:

    1 <= nums.length <= 105
    -104 <= nums[i] <= 104
    k is in the range [1, the number of unique elements in the array].
    It is guaranteed that the answer is unique.

*/

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counted = HashMap::new();
        for num in nums.clone() {
            let count = counted.entry(num).or_insert(0 as i32);
            *count += 1 as i32;
        }
        println!("{:?}", counted);
        let mut counted_vec: Vec<(i32, i32)> = counted.into_iter().collect();
        counted_vec.sort_by(|a, b| b.1.cmp(&a.1));
        counted_vec.into_iter().map(|(id, _count)| id).take(k.try_into().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0347() {
        assert_eq!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
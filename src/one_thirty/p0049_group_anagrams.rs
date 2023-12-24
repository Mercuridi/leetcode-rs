// Leetcode     #0049
// Neetcode 150 #0004

/*
Given an array of strings strs, group the anagrams together. You can return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

 

Example 1:

Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

Example 2:

Input: strs = [""]
Output: [[""]]

Example 3:

Input: strs = ["a"]
Output: [["a"]]

 

Constraints:

    1 <= strs.length <= 104
    0 <= strs[i].length <= 100
    strs[i] consists of lowercase English letters.


 */
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 0 {
            return vec![vec![]]
        }
        let mut grouped: Vec<Vec<String>> = Vec::new();
        for i in 0..strs.len() {
            if grouped.concat().contains(&strs[i]) {
                continue;
            }
            let mut group = vec![strs[i].clone()];
            for j in (i + 1)..strs.len() {
                if is_anagram(strs[i].clone(), strs[j].clone()) {
                    group.push(strs[j].clone());
                }
            }
            grouped.push(group);
            println!("{:?}", grouped);
        }
        grouped
    }
}

// from p0242
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false
    }
    let mut s_letters = HashMap::new();
    let mut t_letters = HashMap::new();
    for i in 0..s.len() {
        let update_letter_s = s_letters.entry(s.chars().nth(i)).or_insert(0);
        let update_letter_t = t_letters.entry(t.chars().nth(i)).or_insert(0);
        *update_letter_s += 1;
        *update_letter_t += 1;
    }
    s_letters == t_letters
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0049() {
        assert_eq!(Solution::group_anagrams(
        vec![
            String::from("eat"), 
            String::from("tea"), 
            String::from("tan"), 
            String::from("ate"), 
            String::from("nat"), 
            String::from("bat")]), 
        vec![
            vec![String::from("eat"), String::from("tea"), String::from("ate")],
            vec![String::from("tan"), String::from("nat")],
            vec![String::from("bat")]]);
        assert_eq!(Solution::group_anagrams(vec![String::from("")]), vec![vec![String::from("")]]);
        assert_eq!(Solution::group_anagrams(vec![String::from("a")]), vec![vec![String::from("a")]]);
    }
}
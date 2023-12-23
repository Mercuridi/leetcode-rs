// Leetcode #242
// Neetcode #2

/*
Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

 

Example 1:

Input: s = "anagram", t = "nagaram"
Output: true

Example 2:

Input: s = "rat", t = "car"
Output: false

Constraints:

    1 <= s.length, t.length <= 5 * 104
    s and t consist of lowercase English letters.

 */

use std::collections::HashMap;
pub struct Solution {}

impl Solution {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_242() {
        assert_eq!(Solution::is_anagram(String::from("anagram"), String::from("nagaram")), true);
        assert_eq!(Solution::is_anagram(String::from("rat"), String::from("car")), false);
        assert_eq!(Solution::is_anagram(String::from("biganagram"), String::from("no")), false);
        assert_eq!(Solution::is_anagram(String::from("biganagram"), String::from("aaabgginrm")), true);
    }
}
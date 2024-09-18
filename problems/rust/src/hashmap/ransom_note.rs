/*
Title: Ransom Note
Link: https://leetcode.com/problems/ransom-note
Difficulty: Easy
Description:
Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
Each letter in magazine can only be used once in ransomNote.

Example 1:
Input: ransomNote = "a", magazine = "b"
Output: false

Example 2:
Input: ransomNote = "aa", magazine = "ab"
Output: false

Example 3:
Input: ransomNote = "aa", magazine = "aab"
Output: true
 
Constraints:
1 <= ransomNote.length, magazine.length <= 105
ransomNote and magazine consist of lowercase English letters.
*/



struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
		let mut magazine_chars: Vec<char> = magazine.chars().collect();
		let ransom_note_chars: Vec<char> = ransom_note.chars().collect();
		
		for letter in ransom_note_chars {
			if let Some(index) = magazine_chars.iter().position(|&m| m == letter) {
				magazine_chars.remove(index);
			} else {
				return false;
			}
		}

		return true;
    }
}


pub fn main() {
    let samples: [(&str, &str); 5] = [
		("a", "b"), // false
		("aa", "ab"), // false
		("aa", "aab"), // true
		("aab", "baa"), // true
		("bg", "efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj") // true
    ];

    for (ransom_note, magazine) in samples {
        let construct: bool = Solution::can_construct(ransom_note.to_string(), magazine.to_string());
        println!("Output: {}", construct);
    }
}


/* 
User Leetcode submission using Rust's Hashmap API

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        
		let mut dict = std::collections::HashMap::new();
        
        for c in magazine.chars() {
            dict.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }
        
        for c in ransom_note.chars() {
            match dict.get_mut(&c) {
                Some(n) if *n > 0 => { *n -= 1; }
                _ => { return false; }
            }
        }
        
        true
    }
}
	
*/

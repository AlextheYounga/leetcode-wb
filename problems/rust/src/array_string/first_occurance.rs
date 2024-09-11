/*  
Title: Find the Index of the First Occurrence in a String
Link: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string
Difficulty: Easy

Description:
Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

Example 1:
Input: haystack = "sadbutsad", needle = "sad"
Output: 0
Explanation: "sad" occurs at index 0 and 6.
The first occurrence is at index 0, so we return 0.

Example 2:
Input: haystack = "leetcode", needle = "leeto"
Output: -1
Explanation: "leeto" did not occur in "leetcode", so we return -1.

Constraints:
1 <= haystack.length, needle.length <= 104
haystack and needle consist of only lowercase English characters.

*/

// Declare `Solution` as an empty struct
struct Solution;

impl Solution {
    // Define a public static method `str_str` that takes two `String` parameters,
    // `haystack` and `needle`, and returns an `i32` (integer).
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // Check if `needle` is an empty string.
        if needle.is_empty() {
        // If `needle` is empty, return 0 as per typical substring search conventions,
        // which usually consider the search of an empty string in any string to always return the index 0.
            return 0;
        }
        // Use the `find` method to search for `needle` in `haystack`.
        // The `find` method returns an `Option<usize>` indicating the index of the first occurrence.
        haystack.find(&needle)
        // Use `map_or` to handle the `Option` result:
        // If `find` returns `None` (needle not found), return -1.
        // If `find` returns `Some(index)`, convert `usize` index to `i32` and return it.
        .map_or(-1, |index| index as i32)
    }
}

fn main() {
    let samples = [
        ["sadbutsad", "sad"],
        ["leetcode", "leeto"]
    ];

    for sample in samples {
        let haystack = String::from(sample[0]);
        let needle = String::from(sample[1]);

        let index = Solution::str_str(haystack, needle);

        println!("Index: {}", index);
    }
}

// To compile: rustc problems/easy/first_occurance.rs
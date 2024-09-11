/*
Title: Single Number II
Link: https://leetcode.com/problems/single-number-ii
Difficulty: Medium

Description:
Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.
You must implement a solution with a linear runtime complexity and use only constant extra space.

Example 1:
Input: nums = [2,2,3,2]
Output: 3

Example 2:
Input: nums = [0,1,0,1,0,1,99]
Output: 99
 

Constraints:
1 <= nums.length <= 3 * 104
-231 <= nums[i] <= 231 - 1
Each element in nums appears exactly three times except for one element which appears once.
*/

struct Solution;

impl Solution {
	// All I had to do was take the best solution we found from the first single number and increase the step increment.
    pub fn single_number(nums: Vec<i32>) -> i32 {
		let mut v = nums;
		v.sort();

        for i in (0..v.len()).step_by(3) {
            if i == v.len() - 1 || v[i] != v[i + 1] {
                return v[i];
            }
        }

        return 0;
    }
}

fn main() {
    let samples: [Vec<i32>; 2] = [
        vec![2,2,3,2], // => 3
        vec![0,1,0,1,0,1,99], // => 99
    ];

    for nums in samples {
        // println!("Input vector: {:?}", nums);
        let single_num: i32 = Solution::single_number(nums);
        println!("Output: {}", single_num);
    }
}

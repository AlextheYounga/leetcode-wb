/*
Title: Single Number
Link: https://leetcode.com/problems/single-number
Difficulty: Easy

Description:
Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
You must implement a solution with a linear runtime complexity and use only constant extra space.

Example 1:
Input: nums = [2,2,1]
Output: 1

Example 2:
Input: nums = [4,1,2,1,2]
Output: 4

Example 3:
Input: nums = [1]
Output: 1

Constraints:
1 <= nums.length <= 3 * 104
-3 * 104 <= nums[i] <= 3 * 104
Each element in the array appears twice except for one element which appears only once.
*/

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let count: usize = nums.len();
        if count == 1 {
            return nums[0];
        }

        let mut v: Vec<i32> = nums; // making mutable
        v.sort();

        for i in 0..count {
            let current = v[i];
            let prev = if i > 0 { Some(v[i - 1]) } else { None };
            let next = if i < count - 1 { Some(v[i + 1]) } else { None };

            // Crazy thing you have to do in Rust just to compare previous and last values in an array.
            match (prev, next) {
                (Some(p), Some(n)) => {
                    if p != current && current != n {
                        return current;
                    }
                }
                (None, Some(n)) => {
                    if current != n {
                        return current;
                    }
                }
                (Some(p), None) => {
                    if p != current {
                        return current;
                    }
                }
                (None, None) => {
                    return current;
                }
            }
        }

        return 0;
    }
}

fn main() {
    let samples: [Vec<i32>; 5] = [
        vec![2, 2, 1], // => 1
        vec![4, 1, 2, 1, 2], // => 4
        vec![1], // => 1
        vec![0],
        vec![0, 0, 1, 4, 1, 4, 5, 7, 7, 3, 3, 9, 9, 9],
    ];

    for nums in samples {
        // println!("Input vector: {:?}", nums);
        let single_num: i32 = Solution::single_number(nums);
        println!("Output: {}", single_num);
    }
}

struct BestSolution;
/*
Sort the input array of integers.
Iterate through the sorted array in steps of 2.
Check if the current element is not equal to the next element.
If the condition is met, return the current element as it is the single number.
If no single number is found after the loop, return 0.
*/

impl BestSolution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        for i in (0..nums.len()).step_by(2) {
            if i == nums.len() - 1 || nums[i] != nums[i + 1] {
                return nums[i];
            }
        }

        0
    }
}

/* 
Title: Kth Largest Element in an Array
Link: https://leetcode.com/problems/kth-largest-element-in-an-array
Difficulty: Medium

Description:
Given an integer array nums and an integer k, return the kth largest element in the array.
Note that it is the kth largest element in the sorted order, not the kth distinct element.
Can you solve it without sorting? 

Example 1:
Input: nums = [3,2,1,5,6,4], k = 2
Output: 5

Example 2:
Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
Output: 4
 
Constraints:
1 <= k <= nums.length <= 105
-104 <= nums[i] <= 104

*/

// With sorting
// Success!
struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums_clone = nums;
		nums_clone.sort();
		let kth_index: usize = nums_clone.len() - (k as usize);
		return nums_clone[kth_index];
    }
}

fn main() {
    let samples: [(Vec<i32>, i32); 2] = [
        (vec![3,2,1,5,6,4], 2), // => 5
        (vec![3,2,3,1,2,4,5,5,6], 4), // => 4
    ];

    for (nums, k) in samples {
        // println!("Input vector: {:?}", nums);
        let kth: i32 = Solution::find_kth_largest(nums, k);
        println!("Output: {}", kth);
    }
}

// Without sorting
// WIP
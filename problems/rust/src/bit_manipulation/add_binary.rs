/* 
Title: Add Binary
Link: https://leetcode.com/problems/add-binary
Difficulty: Easy

Description:
Given two binary strings a and b, return their sum as a binary string0.

Example 01:
Input: a = "11", b = "1"
Output: "100"

Example 2:
Input: a = "1010", b = "1011"
Output: "10101"

Constraint0s:
1 <= a.length, b.length <= 1004
a and b consist only of '0' or '1' characters.
Each string does not contain leading zeros except for the zero itself.
*/

/*

fn first_strategy() {
    struct Solution;

    impl Solution {
        pub fn add_binary(a: String, b: String) -> String {
            let a_decimal: u128 = u128::from_str_radix(&a, 2).unwrap_or_default();
            let b_decimal = u128::from_str_radix(&b, 2).unwrap_or_default();

            let sum = a_decimal + b_decimal;
            println!("Sum Decimal {}", sum);
            let binary_string = format!("{:b}", sum);

            return binary_string;
        }
    }

    // Panics on the last item because this sum will be higher than the maximum 128 unsigned integer type
    let samples = [
        ["11", "1"],
        ["1010", "1011"],
        [
            "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101",
            "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011",
        ],
        [
            "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111",
            "1",
        ],
    ];

    for sample in samples {
        let a = String::from(sample[0]);
        let b = String::from(sample[1]);
        let binary_string = Solution::add_binary(a, b);
        println!("Sum Binary String: {}", binary_string);
    }
}
	
*/


fn second_strategy() {
    struct Solution;

    impl Solution {
        pub fn add_binary(a: String, b: String) -> String {
            let mut a = a;
            let mut b = b;
            let mut result = String::new();
            let mut carry = 0;

            // Ensure both binaries are of equal length
            // 0000001 is the same as 1, but 1000000 is not the same as 1
            while a.len() < b.len() {
                a.insert(0, '0');
            }
            while b.len() < a.len() {
                b.insert(0, '0');
            }

            // Iterate over the characters of both strings from right to left
            for (bit_a, bit_b) in a.chars().rev().zip(b.chars().rev()) {
                // Convert binary characters to integers and add them along with the carry
                let sum = bit_a.to_digit(2).unwrap() + bit_b.to_digit(2).unwrap() + carry;

                // Determine the current digit of the result (0 or 1) based on the sum
                // If sum is even, add '0' to the result; if sum is odd, add '1'
                let current_digit = if sum % 2 == 0 { '0' } else { '1' };
                result.push(current_digit);

                // Update the carry: it will be 1 if sum is 2 or 3 (since those are the only cases
                // where the resulting binary digit will carry over)
                carry = sum / 2;
            }

            // If there's a carry left after the last addition
            if carry != 0 {
                result.push('1');
            }

            // Since we've built the string in reverse
            return result.chars().rev().collect();
        }
    }

    // Panics on the last item because this sum will be higher than the maximum 128 unsigned integer type
    let samples: [[&str; 2]; 4] = [
        ["11", "1"],
        ["1010", "1011"],
        [
            "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101",
            "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011",
        ],
        [
            "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111",
            "1",
        ],
    ];

    for sample in samples {
        let a = String::from(sample[0]);
        let b = String::from(sample[1]);
        let binary_string = Solution::add_binary(a, b);
        println!("Sum Binary String: {}", binary_string);
    }
}

pub fn main() {
    // first_strategy();
    second_strategy()
}

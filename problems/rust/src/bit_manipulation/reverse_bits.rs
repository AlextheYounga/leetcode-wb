/*
Title: Reverse Bits
Link: https://leetcode.com/problems/reverse-bits
Difficulty: Easy

Description:
Reverse bits of a given 32 bits unsigned integer.

Note:
Note that in some languages, such as Java, there is no unsigned integer type. In this case, both input and output will be given as a signed integer type. They should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.
In Java, the compiler represents the signed integers using 2's complement notation. Therefore, in Example 2 above, the input represents the signed integer -3 and the output represents the signed integer -1073741825. 

Example 1:
Input: n = 00000010100101000001111010011100
Output:    964176192 (00111001011110000010100101000000)
Explanation: The input binary string 00000010100101000001111010011100 represents the unsigned integer 43261596, so return 964176192 which its binary representation is 00111001011110000010100101000000.

Example 2:
Input: n = 11111111111111111111111111111101
Output:   3221225471 (10111111111111111111111111111111)
Explanation: The input binary string 11111111111111111111111111111101 represents the unsigned integer 4294967293, so return 3221225471 which its binary representation is 10111111111111111111111111111111.

*/

/*
My Notes: 
Reminder (because I don't always work this close to the CPU):
Unsigned integers are integers that can (basically) only hold non-negative whole numbers. 
Some languages don't have unsigned integers and it's not a problem because the internal representation of the number is the same.

2's complement notation: https://en.wikipedia.org/wiki/Two%27s_complement
*/

struct Solution;

// rust-anal
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
		let bits_string: String = format!("{:032b}", x);
		let string_reversed: String = bits_string.chars().rev().collect::<String>();
		let reversed_binary_number: u32 = u32::from_str_radix(&string_reversed, 2).expect("Invalid binary string");
		return reversed_binary_number;
    }
}

pub fn main() {
    let samples: [&str; 5] = [
		"00000010100101000001111010011100", // 43261596 => 964176192 (00111001011110000010100101000000)
		"11111111111111111111111111111101", // 4294967293 => 3221225471 (10111111111111111111111111111111)
		"00000000000000000000000000000001", // 1 => 2147483648 (10000000000000000000000000000000)
		"11111111111111111111111111111111", // 4294967295 => 4294967295 (11111111111111111111111111111111)
		"10000000000000000000000000000001" // 2147483649 => 2147483649 (10000000000000000000000000000001)
    ];

    for sample in samples {
		let n: u32 = u32::from_str_radix(sample, 2).expect("Invalid binary string");
		println!("Input string, conversion Number {}", n);

        let reversed_int: u32 = Solution::reverse_bits(n);

        println!("Output number: {}", reversed_int);
    }
}

// To compile: rustc problems/easy/reverse_bits.rs


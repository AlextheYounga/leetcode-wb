/*
Write a function that takes the binary representation of a positive integer and returns the number of 
set bits it has (also known as the Hamming weight).

Example 1:
Input: n = 11
Output: 3
Explanation:
The input binary string 1011 has a total of three set bits.

Example 2:
Input: n = 128
Output: 1
Explanation:
The input binary string 10000000 has a total of one set bit.

Example 3:
Input: n = 2147483645
Output: 30
Explanation:
The input binary string 1111111111111111111111111111101 has a total of thirty set bits.

Constraints:
1 <= n <= 231 - 1
 
Follow up: If this function is called many times, how would you optimize it?
*/

struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let bits: String = format!("{:132b}", n);
        let hamming_weight: i32 = bits.matches('1').count() as i32;
        return hamming_weight;
    }
}

fn main() {
    let samples: [i32; 4] = [11, 128, 2147483645, 2147483647];

    /*
	11 => 3
	128 => 1
	2147483645 => 30
	2147483647 => 31 (the largest possible value for i32)
	*/

    for n in samples {
        println!("Input number: {}", n);
        let hamming_weight: i32 = Solution::hamming_weight(n);
        println!("Number hamming weight: {}", hamming_weight);
    }
}

/*
 impl BestSolution {
     pub fn hammingWeight (n: u32) -> i32 {
         (0..32).fold(0, |acc, exp| acc + ((n & (1 << exp)) >> exp)) as i32
     }
 }

 Notes:
 In Rust, the .fold method is an iterator function that allows you to accumulate a single result by iteratively applying a closure or function to each element of an iterator. 
 It is a powerful tool for reducing a collection (like a vector, array, or range) to a single value.

 Example:
 let numbers = vec![1, 2, 3, 4, 5];
 let sum = numbers.iter().fold(0, |acc, &x| acc + x);
 println!("The sum is: {}", sum); // Outputs: "The sum is: 15"
*/

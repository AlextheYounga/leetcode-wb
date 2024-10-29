## Rust Solutions

- [Rust Solutions](#rust-solutions)
- [Number 1 Bits](#number-1-bits)
	- [My Solution](#my-solution)
	- [Best Solution](#best-solution)
- [Reverse Bits](#reverse-bits)
	- [My Answer](#my-answer)
	- [Best Solution](#best-solution-1)
- [Single Number](#single-number)
	- [My Answer](#my-answer-1)
	- [Best Solution](#best-solution-2)
- [First Occurance](#first-occurance)
	- [My Answer](#my-answer-2)
- [Add Binary](#add-binary)
	- [My Answer](#my-answer-3)
- [Bitwise and Numbers Range](#bitwise-and-numbers-range)
	- [My Answer](#my-answer-4)
- [Single Number II](#single-number-ii)
	- [My Answer](#my-answer-5)
- [Ransom Note](#ransom-note)
	- [My Answer](#my-answer-6)
	- [Best Solution](#best-solution-3)
- [Kth Largest Element](#kth-largest-element)
	- [My Answer](#my-answer-7)


## Number 1 Bits
**Question:** Write a function that takes the binary representation of a positive integer and returns the number of 
set bits it has (also known as the Hamming weight). 

### My Solution
```rs
struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let bits: String = format!("{:132b}", n);
        let hamming_weight: i32 = bits.matches('1').count() as i32;
        return hamming_weight;
    }
}
```

### Best Solution
```rs
impl BestSolution {
	pub fn hammingWeight (n: u32) -> i32 {
		(0..32).fold(0, |acc, exp| acc + ((n & (1 << exp)) >> exp)) as i32
	}
}
```

>Note: In Rust, the `.fold` method is an iterator function that allows you to accumulate a single result by iteratively applying a closure or function to each element of an iterator. It is a powerful tool for reducing a collection (like a vector, array, or range) to a single value.

Example:
```rs
	let numbers = vec![1, 2, 3, 4, 5];
	let sum = numbers.iter().fold(0, |acc, &x| acc + x);
	println!("The sum is: {}", sum); // Outputs: "The sum is: 15"
```

## Reverse Bits
**Question:** Reverse bits of a given 32 bits unsigned integer.

### My Answer
```rs
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
		let bits_string: String = format!("{:032b}", x);
		let string_reversed: String = bits_string.chars().rev().collect::<String>();
		let reversed_binary_number: u32 = u32::from_str_radix(&string_reversed, 2).expect("Invalid binary string");
		return reversed_binary_number;
    }
}
```

### Best Solution
The best submitted solution lmao
"looked up the docs, crazy"
```rs
impl BestSolution {
	pub fn reverse_bits(x: u32) -> u32 {
		x.reverse_bits()
	}
}
```

## Single Number
**Question:** Sort the input array of integers. Iterate through the sorted array in steps of 2. Check if the current element is not equal to the next element. If the condition is met, return the current element as it is the single number. If no single number is found after the loop, return 0.

### My Answer
```rs
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
```

### Best Solution
```rs
struct BestSolution;

impl BestSolution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        nums.sort()
        for i in (0..nums.len()).step_by(2) {
            if i == nums.len() - 1 || nums[i] != nums[i + 1] {
                return nums[i];
            }
        0
    	}
	}
}
```

## First Occurance
**Question:** Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

### My Answer
```rs
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
```

## Add Binary
**Question:** Given two binary strings a and b, return their sum as a binary string0.

### My Answer
```rs
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
```

## Bitwise and Numbers Range
**Question:** Given two integers left and right that represent the range [left, right], return the bitwise AND of all numbers in this range, inclusive.

### My Answer
```rs
struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
		let left = left;
		let mut right = right;
		
		while left < right {
			right = right & (right - 1);  // Remove the lowest set bit of right
		}
	
		return left & right;
    }
}
```


## Single Number II
**Question:** Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it. You must implement a solution with a linear runtime complexity and use only constant extra space.

### My Answer
```rs
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
```

## Ransom Note
**Question:** Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise. Each letter in magazine can only be used once in ransomNote.

### My Answer
```rs
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

```

### Best Solution
```rs
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
```

## Kth Largest Element
**Question:** Given an integer array nums and an integer k, return the kth largest element in the array. Note that it is the kth largest element in the sorted order, not the kth distinct element.

### My Answer
```rs
struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums_clone = nums;
		nums_clone.sort();
		let kth_index: usize = nums_clone.len() - (k as usize);
		return nums_clone[kth_index];
    }
}
```

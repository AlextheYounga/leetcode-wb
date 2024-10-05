## Rust Best User Submitted Solutions

## Number  1 Bits
```rs
impl BestSolution {
	pub fn hammingWeight (n: u32) -> i32 {
		(0..32).fold(0, |acc, exp| acc + ((n & (1 << exp)) >> exp)) as i32
	}
}
```

In Rust, the `.fold` method is an iterator function that allows you to accumulate a single result by iteratively applying a closure or function to each element of an iterator. 
It is a powerful tool for reducing a collection (like a vector, array, or range) to a single value.

Example:
```rs
	let numbers = vec![1, 2, 3, 4, 5];
	let sum = numbers.iter().fold(0, |acc, &x| acc + x);
	println!("The sum is: {}", sum); // Outputs: "The sum is: 15"
```

## Reverse Bits
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
Sort the input array of integers.
Iterate through the sorted array in steps of 2.
Check if the current element is not equal to the next element.
If the condition is met, return the current element as it is the single number.
If no single number is found after the loop, return 0.

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
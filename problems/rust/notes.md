# Rust Notes

## Finding unique values using HashSet

```rust
    pub fn unique_values(nums: Vec<i32>) -> i32 {
		use std::collections::HashSet;
		let unique_numbers: HashSet<i32> = nums.into_iter().collect();
		let mut unique_numbers_vec: Vec<i32> = unique_numbers.into_iter().collect();

		// This is kind of dumb
		if let Some(single_number) = unique_numbers_vec.pop() {
			return single_number;
		}
		return 0;
	}
```

## Finding unique values using vectors

```rust
    pub fn unique_values(nums: Vec<i32>) -> i32 {
		let mut unique_values = Vec::new();

		for &num in &nums {
			if !unique_values.contains(&num) {
				unique_values.push(num);
			}
		}

		return unique_values[0];
	}
```


## Making a immutable variable mutable
This seems to be the only way
```rust
let x = 1;
let mut y = x;
```

## Tertiary nullable values
```rust
let last: Option<i32> = if i != 0 { Some(v[i - 1]) } else { None };
```
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

## Strings
By default, all string literals are stored in static memory allocation at compile time, (given the type `&str`) meaning they will be stored in memory for the entire duration of the program, (as opposed to dynamically allocated memory which is added to the heap at runtime). 

```rust
let mystring: &str = "some string";
```

`String` is a heap-allocated, mutable, and growable string type.  It allows you to change its size and contents over time by appending, removing, or modifying characters. A `String` owns its data and is responsible for managing the underlying memory. This gives it more flexibility but also adds overhead in terms of heap allocation and memory management. You can easily convert a `String` to a `&str` by borrowing a slice of it (e.g., `&my_string`).

```rust
let mutable_str = String::from("hello world");
let mutable_str = "hello world".to_string();
let empty_string = String::new();
```

### Sorting
```rust
// Convert the string into a vector of characters
let mut ransom_note_chars: Vec<char> = ransom_note.chars().collect();
let mut magazine_chars: Vec<char> = magazine.chars().collect();

// Sort the vector of characters
ransom_note_chars.sort();
magazine_chars.sort();

// Collect the sorted characters back into a string
let sorted_ransom_note: String = ransom_note_chars.into_iter().collect();
let sorted_magazine: String = magazine_chars.into_iter().collect();
```
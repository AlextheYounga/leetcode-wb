mod bit_manipulation;
mod array_string;
mod heap;
mod hashmap;

fn main() {
	// Using here so my Rust linter doesn't complain about dead code. 
	use bit_manipulation::add_binary::main as add_binary;
	use bit_manipulation::bitwise_and_numbers_range::main as bitwise_and_numbers_range;
	use bit_manipulation::number_1_bits::main as number_1_bits;
	use bit_manipulation::reverse_bits::main as reverse_bits;
	use bit_manipulation::single_number::main as single_number;
	use bit_manipulation::single_number_ii::main as single_number_ii;
	use array_string::first_occurance::main as first_occurance;
	use hashmap::ransom_note::main as ransom_note;
	use heap::kth_largest_element::main as kth_largest_element;
    
	add_binary();
	bitwise_and_numbers_range();
	number_1_bits();
	reverse_bits();
	single_number();
	single_number_ii();
	first_occurance();
	ransom_note();
	kth_largest_element();

	println!("Hello, world!");
}

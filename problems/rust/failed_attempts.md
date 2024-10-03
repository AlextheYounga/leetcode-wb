# Rust Failed Attempts

## Add Binary
```rs
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
```
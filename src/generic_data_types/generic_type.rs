fn greatest<T: std::cmp::PartialOrd>(input: &[T]) -> &T {
    let mut result = &input[0];

    for i in input {
        if i > result {
            result = i;
        }
    }

    result
}

fn main() {
    let nums = vec![1111, 1, 2, 3, 9, 12];

    let greatest_number = greatest(&nums);
    println!("The greatest number is {}", greatest_number);

    let chars = vec!['Z', 'A', 'B', 'G', 'K'];

    let greatest_character = greatest(&chars);
    println!("The greatest character is {}", greatest_character);
}
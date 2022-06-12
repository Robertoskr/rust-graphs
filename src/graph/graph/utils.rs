use std::io::stdin;

pub fn read_numbers() -> Vec<i64> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    buffer
        .split_whitespace()
        .map(|word| word.parse::<i64>().unwrap())
        .collect()
}

pub fn read_two_numbers() -> (i64, i64) {
    let numbers = read_numbers();
    (numbers[0], numbers[1])
}

pub fn read_one_number() -> i64 {
    let numbers = read_numbers();
    numbers[0]
}

pub fn read_three_numbers() -> (i64, i64, i64) {
    let numbers = read_numbers();
    (numbers[0], numbers[1], numbers[2])
}

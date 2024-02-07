use std::fs::read_to_string;

fn main() {
    let filename = r"d:\dev\Projects_Rust\advent_app_data\2024_01_A_input.txt";
    let lines = read_lines(filename);
    let result = lines.into_iter().map(|x| find_digit_in_string(&x)).sum::<u32>();

    println!("{}", result)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn find_digit_in_string(string: &str) -> u32 {
    let mut dig = 0;

    for c in string.chars() {
        if c.is_digit(10) {
            dig = 10 * c.to_digit(10).unwrap();
            break;
        }
    }

    for c in string.chars().rev() {
        if c.is_digit(10) {
            dig += c.to_digit(10).unwrap();
            break;
        }
    }

    dig
}

use std::fs::read_to_string;

fn main() {
    let filename = r"..\advent_app_data\2024_01_A_input.txt";
    let lines = read_lines(filename);
    let result = lines
        .into_iter()
        .map(|x| find_digit_in_string(&x))
        .sum::<u32>();

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
    let val1 = get_digit(string.chars());
    let val2 = get_digit(string.chars().rev());
    val1 * 10 + val2
}

fn get_digit<T>(chars: T) -> u32
where
    T: IntoIterator<Item = char>,
{
    chars
        .into_iter()
        .filter(|c| c.is_digit(10))
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap()
}

use crate::helpers;

#[allow(dead_code)]
fn run(filename: &str) -> u32 {
    helpers::read_lines(filename)
        .into_iter()
        .map(|x| find_digit_in_string(&x))
        .sum::<u32>()
}

fn find_digit_in_string(string: &str) -> u32 {
    let val1 = get_digit(string.chars());
    let val2 = get_digit(string.chars().rev());
    val1 * 10 + val2
}

fn get_digit<T>(chars: T) -> u32
where
    T: Iterator<Item = char>,
{
    chars
        .filter(|c| c.is_digit(10))
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2023_01_a_sample() {
        let actual = run(r"..\advent_app_data\2024_01_A_sample.txt");
        assert_eq!(actual, 142);
    }

    #[test]
    fn test_2023_01_a_input() {
        let actual = run(r"..\advent_app_data\2024_01_A_input.txt");
        assert_eq!(actual, 54644);
    }
}

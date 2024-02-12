use std::collections::HashMap;

use crate::helpers;
use crate::helpers::IteratorExt;

#[allow(dead_code)]
fn run_a(filename: &str) -> u32 {
    helpers::read_lines(filename)
        .into_iter()
        .map(|x| process_a(&x))
        .sum::<u32>()
}

fn process_a(string: &str) -> u32 {
    let val1 = get_digit(string.chars());
    let val2 = get_digit(string.chars().rev());
    val1 * 10 + val2
}

fn get_digit(chars: impl Iterator<Item = char>) -> u32 {
    chars
        .filter(|c| c.is_digit(10))
        .first()
        .to_digit(10)
        .unwrap()
}

#[allow(dead_code)]
fn run_b(filename: &str) -> u32 {
    helpers::read_lines(filename)
        .into_iter()
        .map(|x| process_b(&x))
        .sum::<u32>()
}

fn process_b(line: &str) -> u32 {
    let dic = HashMap::from([
        ("one", 1u32),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    fn filter<'a>(
        iter: impl Iterator<Item = (Option<usize>, &'a u32)>,
    ) -> impl Iterator<Item = (usize, &'a u32)> {
        iter.filter(|(a, _)| a.is_some())
            .map(|(a, b)| (a.unwrap(), b))
    }

    let val1 = filter(dic.iter().map(|(a, b)| (line.find(a), b)))
        .min_by_key(|a| a.0)
        .unwrap()
        .1;

    let val2 = filter(dic.iter().map(|(a, b)| (line.rfind(a), b)))
        .max_by_key(|a| a.0)
        .unwrap()
        .1;

    val1 * 10 + val2
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use super::run_a;
    use super::run_b;
    use crate::test_base::get_test_file_name;

    #[test]
    fn test_2023_01_A_sample() {
        let actual = run_a(&get_test_file_name(2023, 1, "sampleA"));
        assert_eq!(actual, 142);
    }

    #[test]
    fn test_2023_01_A_input() {
        let actual = run_a(&get_test_file_name(2023, 1, "input"));
        assert_eq!(actual, 54644);
    }

    #[test]
    fn test_2023_01_B_sample() {
        let actual = run_b(&get_test_file_name(2023, 1, "sampleB"));
        assert_eq!(actual, 281);
    }

    #[test]
    fn test_2023_01_B_input() {
        let actual = run_b(&get_test_file_name(2023, 1, "input"));
        assert_eq!(actual, 53348);
    }
}

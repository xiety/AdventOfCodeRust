use crate::helpers;
use crate::helpers::IteratorExt;
use std::collections::HashMap;

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
        .filter(|c| c.is_ascii_digit())
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
    use super::{run_a, run_b};
    use crate::test_base::test::run_test;

    #[test]
    fn a_sample() {
        run_test(file!(), "sampleA", run_a, 142);
    }

    #[test]
    fn a_input() {
        run_test(file!(), "input", run_a, 54644);
    }

    #[test]
    fn b_sample() {
        run_test(file!(), "sampleB", run_b, 281);
    }

    #[test]
    fn b_input() {
        run_test(file!(), "input", run_b, 53348);
    }
}

use crate::helpers;
use crate::helpers::IteratorExt;

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
        .first()
        .to_digit(10)
        .unwrap()
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use super::run;
    use crate::test_base::get_test_file_name;

    #[test]
    fn test_2023_01_A_sample() {
        let actual = run(&get_test_file_name(2023, 1, "A", false));
        assert_eq!(actual, 142);
    }

    #[test]
    fn test_2023_01_A_input() {
        let actual = run(&get_test_file_name(2023, 1, "A", true));
        assert_eq!(actual, 54644);
    }
}

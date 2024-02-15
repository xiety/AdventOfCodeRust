use crate::helpers::IteratorExt;
use crate::lines_map::LinesMap;
use regex_lite::{Captures, Regex};

#[allow(dead_code)]
fn run_a(filename: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let lines = LinesMap::load(filename);

    lines
        .enumerate_lines()
        .flat_map(|(index, line)| {
            let lines = &lines;
            let cy = index as i32;
            iterate_captures(&re, line).filter_map(move |(from, to, num)| {
                lines
                    .chars2d(from - 1, to + 1, cy - 1, cy + 2)
                    .any(|(c, _, _)| !c.is_ascii_digit() && c != '.')
                    .then_some(num)
            })
        })
        .sum()
}

#[allow(dead_code)]
fn run_b(filename: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let lines = LinesMap::load(filename);

    lines
        .enumerate_lines()
        .flat_map(|(index, line)| {
            let lines = &lines;
            let cy = index as i32;
            iterate_captures(&re, line).flat_map(move |(from, to, num)| {
                lines
                    .chars2d(from - 1, to + 1, cy - 1, cy + 2)
                    .filter_map(move |(c, x, y)| (c == '*').then_some((num, x, y)))
            })
        })
        .group_by(|x| (x.1, x.2))
        .into_iter()
        .filter(|bt| bt.1.len() == 2)
        .map(|bt| bt.1.into_iter().map(|w| w.0).product::<u32>())
        .sum()
}

fn iterate_captures<'a>(
    re: &'a Regex,
    line: &'a str,
) -> impl Iterator<Item = (i32, i32, u32)> + 'a {
    re.captures_iter(line).map(get_capture)
}

fn get_capture(c: Captures<'_>) -> (i32, i32, u32) {
    let m = c.get(0).unwrap();
    let from = m.start() as i32;
    let to = m.end() as i32;
    let num: u32 = m.as_str().parse().unwrap();

    (from, to, num)
}

#[cfg(test)]
mod test {
    use super::{run_a, run_b};
    use crate::test_base::get_test_file_name;

    #[test]
    fn a_sample() {
        let actual = run_a(&get_test_file_name(2023, 3, "sample"));
        assert_eq!(actual, 4361);
    }

    #[test]
    fn a_input() {
        let actual = run_a(&get_test_file_name(2023, 3, "input"));
        assert_eq!(actual, 539713);
    }

    #[test]
    fn b_sample() {
        let actual = run_b(&get_test_file_name(2023, 3, "sample"));
        assert_eq!(actual, 467835);
    }

    #[test]
    fn b_input() {
        let actual = run_b(&get_test_file_name(2023, 3, "input"));
        assert_eq!(actual, 84159075);
    }
}

use std::{cmp::{max, min}, ops::Range};

use crate::helpers::read_lines;
use regex_lite::{Captures, Regex};

#[allow(dead_code)]
fn run_a(filename: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let lines = read_lines(filename);

    let (width, height) = get_size(&lines);

    lines
        .iter()
        .enumerate()
        .flat_map(|(index, line)| {
            let lines = &lines;
            re.captures_iter(line.as_str())
                .filter_map(move |c| {
                    let (from, to, num) = get_capture(c);
                    let cy = index as i32;

                    for2d(
                        max(0, from - 1)..min(width, to + 1),
                        max(0, cy - 1)..min(height, cy + 2),
                    )
                    .map(|(x, y)| get_char(&lines, x, y))
                    .any(|p| !p.is_digit(10) && p != '.')
                    .then_some(num)
                })
        })
        .sum()
}

fn get_capture(c: Captures<'_>) -> (i32, i32, u32) {
    let m = c.get(0).unwrap();
    let from = m.start() as i32;
    let to = m.end() as i32;
    let num: u32 = m.as_str().parse().unwrap();

    (from, to, num)
}

fn get_char(lines: &Vec<String>, x: i32, y: i32) -> char {
    lines[y as usize].chars().nth(x as usize).unwrap()
}

fn for2d(rx: Range<i32>, ry: Range<i32>) -> impl Iterator<Item = (i32, i32)> {
    ry.flat_map(move |y| rx.clone().map(move |x| (x, y)))
}

fn get_size(lines: &Vec<String>) -> (i32, i32) {
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    (width, height)
}

#[allow(dead_code)]
fn run_b(filename: &str) -> u32 {
    0
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

    /*#[test]
    fn b_sample() {
        let actual = run_b(&get_test_file_name(2023, 3, "sample"));
        assert_eq!(actual, 467835);
    }

    #[test]
    fn b_input() {
        let actual = run_b(&get_test_file_name(2023, 3, "input"));
        assert_eq!(actual, 84159075);
    }*/
}

use std::iter::Enumerate;

use crate::helpers::read_lines;
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
            re.captures_iter(line.as_str())
                .map(get_capture)
                .filter_map(move |(from, to, num)| {
                    lines
                        .chars2d(from - 1, to + 1, cy - 1, cy + 2)
                        .any(|p| !p.is_ascii_digit() && p != '.')
                        .then_some(num)
                })
        })
        .sum()
}

struct LinesMap {
    lines: Vec<String>,
}

impl LinesMap {
    fn for2d(&self, fx: i32, tx: i32, fy: i32, ty: i32) -> impl Iterator<Item = (i32, i32)> {
        let (width, height) = self.get_size();
        (fy..ty)
            .flat_map(move |y| (fx..tx).map(move |x| (x, y)))
            .filter(move |(x, y)| x >= &0 && x < &width && y >= &0 && y < &height)
    }

    fn chars2d(&self, fx: i32, tx: i32, fy: i32, ty: i32) -> impl Iterator<Item = char> + '_ {
        self.for2d(fx, tx, fy, ty).map(move |(x, y)| self.get_char(x, y))
    }

    fn load(filename: &str) -> LinesMap {
        LinesMap {
            lines: read_lines(filename),
        }
    }

    fn get_size(&self) -> (i32, i32) {
        let width = self.lines[0].len() as i32;
        let height = self.lines.len() as i32;
        (width, height)
    }

    fn get_char(&self, x: i32, y: i32) -> char {
        self.lines[y as usize].chars().nth(x as usize).unwrap()
    }

    fn enumerate_lines(&self) -> Enumerate<std::slice::Iter<String>> {
        self.lines.iter().enumerate()
    }
}

fn get_capture(c: Captures<'_>) -> (i32, i32, u32) {
    let m = c.get(0).unwrap();
    let from = m.start() as i32;
    let to = m.end() as i32;
    let num: u32 = m.as_str().parse().unwrap();

    (from, to, num)
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

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
                    .any(|c| !c.is_ascii_digit() && c != '.')
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
                    .enumerate_chars2d(from - 1, to + 1, cy - 1, cy + 2)
                    .filter_map(move |(c, x, y)| (c == '*').then_some((num, x, y)))
            })
        })
        .group_by(|&(_num, x, y)| (x, y))
        .filter(|(_key, list)| list.len() == 2)
        .map(|(_key, list)| list.into_iter().map(|(num, _x, _y)| num).product::<u32>())
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
    use crate::test_base::test::run_test;

    #[test]
    fn a_sample() {
        run_test(file!(), "sample", run_a, 4361);
    }

    #[test]
    fn a_input() {
        run_test(file!(), "input", run_a, 539713);
    }

    #[test]
    fn b_sample() {
        run_test(file!(), "sample", run_b, 467835);
    }

    #[test]
    fn b_input() {
        run_test(file!(), "input", run_b, 84159075);
    }
}

use std::cmp::{max, min};

use crate::helpers::read_lines;
use regex_lite::Regex;

#[allow(dead_code)]
fn run_a(filename: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let lines = read_lines(filename);

    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let mut result = 0;

    for (index, line) in lines.iter().enumerate() {
        let cy = index as i32;

        for c in re.captures_iter(line.as_str()) {
            let m = c.get(0).unwrap();
            let from = m.start() as i32;
            let to = m.end() as i32;
            let num: u32 = m.as_str().parse().unwrap();

            let mut part = false;

            'y: for y in max(0, cy - 1)..min(height, cy + 2) {

                let subline = &lines[y as usize];

                for x in max(0, from - 1)..min(width, to + 1) {

                    let p = subline.chars().nth(x as usize).unwrap();

                    if !p.is_digit(10) && p != '.' {
                        part = true;
                        break 'y;
                    }
                }
            }

            if part {
                result += num;
            }
        }
    }

    result
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

use crate::helpers::{read_parsed, StringExt};
use macros::macro_regex;

#[allow(dead_code)]
fn run_a(filename: &str) -> u32 {
    read_parsed::<R1>(filename)
        .into_iter()
        .map(parse)
        .map(|c| {
            match c
                .left
                .into_iter()
                .filter(move |cl| c.right.contains(&cl))
                .count() as u32
            {
                0 => 0,
                w => 2u32.pow(w - 1),
            }
        })
        .sum()
}

fn parse(r1: R1) -> Card {
    Card {
        num: r1.num,
        left: r1.left.split_to_array(),
        right: r1.right.split_to_array(),
    }
}

#[allow(dead_code)]
fn run_b(filename: &str) -> u32 {
    0
}

#[macro_regex(r"^Card\s+(?<num>\d+):\s+(?<left>.+)\s+\|\s+(?<right>.+)$")]
#[derive(Debug)]
struct R1 {
    num: u32,
    left: String,
    right: String,
}

#[derive(Debug)]
struct Card {
    num: u32,
    left: Vec<u32>,
    right: Vec<u32>,
}

#[cfg(test)]
mod test {
    use super::{run_a, run_b};
    use crate::test_base::test::run_test;

    #[test]
    fn a_sample() {
        run_test(file!(), "sample", run_a, 13);
    }

    #[test]
    fn a_input() {
        run_test(file!(), "input", run_a, 26426);
    }

    #[test]
    fn b_sample() {
        run_test(file!(), "sample", run_b, 30);
    }

    #[test]
    fn b_input() {
        run_test(file!(), "input", run_b, 6227972);
    }
}

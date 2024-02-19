use std::cmp::min;

use crate::helpers::{read_parsed, StringExt};
use macros::macro_regex;

#[allow(dead_code)]
fn run_a(filename: &str) -> u32 {
    load(filename)
        .map(|x| match calc_win(x) {
            0 => 0,
            w => 2_u32.pow(w - 1),
        })
        .sum()
}

fn calc_win(c: Card) -> u32 {
    c.left
        .into_iter()
        .filter(move |cl| c.right.contains(&cl))
        .count() as u32
}

fn load(filename: &str) -> impl Iterator<Item = Card> {
    read_parsed::<R1>(filename).into_iter().map(|x| Card {
        left: x.left.split_to_array(),
        right: x.right.split_to_array(),
    })
}

#[allow(dead_code)]
pub fn run_b(filename: &str) -> u32 {
    let mut calculates: Vec<Calc> = load(filename)
        .map(move |c| Calc {
            win: calc_win(c),
            copies: 1,
        })
        .collect();

    for i in 0..calculates.len() {
        for j in (i + 1)..min(i + 1 + calculates[i].win as usize, calculates.len()) {
            calculates[j].copies += calculates[i].copies;
        }
    }

    calculates.into_iter().map(|x| x.copies).sum()
}

#[macro_regex(r"^Card\s+\d+:\s+(?<left>.+)\s+\|\s+(?<right>.+)$")]
#[derive(Debug)]
struct R1 {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Card {
    left: Vec<u32>,
    right: Vec<u32>,
}

struct Calc {
    win: u32,
    copies: u32,
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

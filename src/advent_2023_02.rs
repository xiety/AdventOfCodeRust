use crate::helpers::{self, IteratorExt};

use macros::macro_regex;

#[allow(dead_code)]
fn run_a(filename: &str) -> u32 {
    let items = load(filename);

    let cubes = [12, 13, 14];

    items
        .into_iter()
        .filter_map(|x| {
            if x.balls.into_iter().any(|w| (0..3).any(|i| cubes[i] < w[i])) {
                None
            } else {
                Some(x.num)
            }
        })
        .sum()
}

#[allow(dead_code)]
fn run_b(filename: &str) -> u32 {
    let items = load(filename);

    items
        .into_iter()
        .map(|x| {
            (0..3)
                .map(|i| x.balls.iter().map(|x| x[i]).max().unwrap())
                .product::<u32>()
        })
        .sum()
}

fn parse(line: &str) -> Game {
    let r1: R1 = line.parse().unwrap();

    let balls = parse_game_data(r1.data.as_str());

    Game { num: r1.num, balls }
}

fn parse_game_data(line: &str) -> Vec<[u32; 3]> {
    let colors = ["red", "green", "blue"];

    let tosses = line.split("; ");

    tosses
        .map(|toss| {
            let mut balls = [0, 0, 0];

            let throws = toss.split(", ");

            for throw in throws {
                let r2: R2 = throw.parse().unwrap();

                let index = colors.iter().index_of(&r2.color.as_str());
                balls[index] = r2.num;
            }

            balls
        })
        .collect()
}

fn load(filename: &str) -> Vec<Game> {
    helpers::read_lines(filename)
        .into_iter()
        .map(|x| parse(&x))
        .collect()
}

#[derive(Debug)]
struct Game {
    num: u32,
    balls: Vec<[u32; 3]>,
}

#[macro_regex(r"^Game (?<num>\d+): (?<data>.*)$")]
struct R1 {
    num: u32,
    data: String,
}

#[macro_regex(r"^(?<num>\d+) (?<color>\w+)$")]
struct R2 {
    num: u32,
    color: String,
}

#[cfg(test)]
mod test {
    use super::{run_a, run_b};
    use crate::test_base::get_test_file_name;

    #[test]
    fn a_sample() {
        let actual = run_a(&get_test_file_name(2023, 2, "sample"));
        assert_eq!(actual, 8);
    }

    #[test]
    fn a_input() {
        let actual = run_a(&get_test_file_name(2023, 2, "input"));
        assert_eq!(actual, 3059);
    }

    #[test]
    fn b_sample() {
        let actual = run_b(&get_test_file_name(2023, 2, "sample"));
        assert_eq!(actual, 2286);
    }

    #[test]
    fn b_input() {
        let actual = run_b(&get_test_file_name(2023, 2, "input"));
        assert_eq!(actual, 65371);
    }
}

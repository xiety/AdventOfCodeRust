use crate::helpers::{self, CapturesExt, IteratorExt};

use regex_lite::Regex;

#[allow(dead_code)]
fn run_a(filename: &str) -> u32 {
    let items = load(filename);

    let cubes = [12, 13, 14];

    items
        .into_iter()
        .filter_map(|x| {
            for toss in x.balls {
                if (0..3).any(|i| cubes[i] < toss[i]) {
                    return None;
                }
            }
            Some(x.num)
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
    let re = Regex::new(r"^Game (?<GameNumber>\d+): (?<GameData>.*)$").unwrap();

    let caps = re.captures(line).unwrap();
    let num: u32 = caps.get_type("GameNumber");
    let game_data = caps.get_str("GameData");

    let balls = parse_game_data(game_data);

    Game { num, balls }
}

fn parse_game_data(line: &str) -> Vec<[u32; 3]> {
    let colors = ["red", "green", "blue"];

    let re = Regex::new(r"^(?<Num>\d+) (?<Color>\w+)$").unwrap();

    let tosses = line.split("; ");

    tosses
        .map(|toss| {
            let mut balls = [0, 0, 0];

            let throws = toss.split(", ");

            for throw in throws {
                let caps = re.captures(throw).unwrap();

                let num: u32 = caps.get_type("Num");
                let color = caps.get_str("Color");

                let index = colors.iter().index_of(&color);
                balls[index] = num;
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

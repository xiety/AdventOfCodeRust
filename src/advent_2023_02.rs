use crate::helpers::{self, CapturesExt};

use regex_lite::Regex;

#[allow(dead_code)]
fn run_a(filename: &str) -> u32 {
    let items = load(filename);

    let cubes = [12, 13, 14];

    let mut result = 0;

    for item in items {
        let mut good = true;

        'toss: for toss in item.balls {
            for i in 0..3 {
                if cubes[i] < toss[i] { 
                    good = false;
                    break 'toss;
                }
            }
        }

        if good { result += item.num; }
    }

    result
}

fn parse(line: &str) -> Game {
    let re = Regex::new(r"^Game (?<GameNumber>\d+): (?<GameData>.*)$").unwrap();

    let caps = re.captures(line).unwrap();
    let game_num: u32 = caps.get_type("GameNumber");
    let game_data = caps.get_str("GameData");

    let balls = parse_game_data(game_data);

    Game {
        num: game_num,
        balls: balls,
    }
}

fn parse_game_data(line: &str) -> Vec<[u32; 3]> {
    let colors = ["red", "green", "blue"];

    let re = Regex::new(r"^(?<Num>\d+) (?<Color>\w+)$").unwrap();

    let tosses = line.split("; ");

    let mut list: Vec<[u32; 3]> = Vec::new();

    for toss in tosses {
        let mut balls = [0, 0, 0];

        let throws = toss.split(", ");

        for throw in throws {
            let caps = re.captures(throw).unwrap();

            let num: u32 = caps.get_type("Num");
            let color = caps.get_str("Color");

            let index = colors.iter().position(|&x| x == color).unwrap();
            balls[index] = num;
        }

        list.push(balls)
    }

    list
}

#[allow(dead_code)]
fn run_b(filename: &str) -> u32 {
    let items = load(filename);

    let mut result = 0;

    for item in items {
        let mut mul = 1;

        for i in 0..3 {
            mul *= item.balls.iter().map(|x| x[i]).max().unwrap();
        }

        result += mul;
    }

    result
}

fn load(filename: &str) -> Vec<Game> {
    helpers::read_lines(filename)
        .into_iter()
        .map(|x| parse(&x))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Game {
    num: u32,
    balls: Vec<[u32; 3]>,
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use super::run_a;
    use super::run_b;
    use crate::test_base::get_test_file_name;

    #[test]
    fn A_sample() {
        let actual = run_a(&get_test_file_name(2023, 2, "sample"));
        assert_eq!(actual, 8);
    }

    #[test]
    fn A_input() {
        let actual = run_a(&get_test_file_name(2023, 2, "input"));
        assert_eq!(actual, 3059);
    }

    #[test]
    fn B_sample() {
        let actual = run_b(&get_test_file_name(2023, 2, "sample"));
        assert_eq!(actual, 2286);
    }

    #[test]
    fn B_input() {
        let actual = run_b(&get_test_file_name(2023, 2, "input"));
        assert_eq!(actual, 65371);
    }
}

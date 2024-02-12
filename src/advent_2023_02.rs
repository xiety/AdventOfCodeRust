use crate::helpers::{self, CapturesExt};

use regex_lite::Regex;

#[allow(dead_code)]
fn run_a(filename: &str) -> u32 {
    let _items = helpers::read_lines(filename).into_iter().map(|x| parse(&x)).collect::<Vec<_>>();
    0
}

fn parse(line: &str) -> Game {
    let re = Regex::new(r"^Game (?<GameNumber>\d+): (?<GameData>.*)$").unwrap();

    let caps = re.captures(line).unwrap();
    let game_num: u32 = caps.get_type("GameNumber");
    let game_data: &str = caps.get_type("GameData");

    Game {
        num: game_num,
        //balls: Vec::from(0),
    }
}

#[derive(Debug)]
struct Game {
    num: u32,
    //balls: Vec<[u32; 3]>,
}

#[allow(dead_code)]
fn run_b(filename: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use super::run_a;
    use super::run_b;
    use crate::test_base::get_test_file_name;

    #[test]
    fn test_2023_01_A_sample() {
        let actual = run_a(&get_test_file_name(2023, 2, "sample"));
        assert_eq!(actual, 8);
    }

    #[test]
    fn test_2023_01_A_input() {
        let actual = run_a(&get_test_file_name(2023, 2, "input"));
        assert_eq!(actual, 3059);
    }

    #[test]
    fn test_2023_01_B_sample() {
        let actual = run_b(&get_test_file_name(2023, 2, "sample"));
        assert_eq!(actual, 2286);
    }

    #[test]
    fn test_2023_01_B_input() {
        let actual = run_b(&get_test_file_name(2023, 2, "input"));
        assert_eq!(actual, 65371);
    }
}

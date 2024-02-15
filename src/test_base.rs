#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
pub fn get_test_file_name(year: u32, number: u32, part: &str) -> String {
    format!("..\\advent_app_data\\{year}_{number:0>2}_{part}.txt")
}

#[cfg(test)]
pub fn run_test<T, F>(year: u32, number: u32, part: &str, f: F, expected: T)
where
    T: PartialEq + Debug,
    F: Fn(&str) -> T,
{
    let filename = format!("..\\advent_app_data\\{year}_{number:0>2}_{part}.txt");
    let actual = f(&filename);
    assert_eq!(actual, expected);
}

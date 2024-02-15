#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
pub fn run_test<T, F>(code_file: &str, part: &str, f: F, expected: T)
where
    T: PartialEq + Debug,
    F: Fn(&str) -> T,
{
    use std::path::Path;

    let code_filename_only = Path::new(code_file).file_stem().and_then(|s| s.to_str()).unwrap();
    let filename = format!("..\\advent_app_data\\{code_filename_only}_{part}.txt");
    let actual = f(&filename);
    assert_eq!(actual, expected);
}

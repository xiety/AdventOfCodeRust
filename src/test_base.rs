#[cfg(test)]
pub mod test {

    use std::fmt::Debug;
    use std::path::Path;

    pub fn run_test<T, F>(code_file: &str, part: &str, f: F, expected: T)
    where
        T: PartialEq + Debug,
        F: Fn(&str) -> T,
    {
        let code_filename_only = Path::new(code_file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap();
        let filename = format!("data\\{code_filename_only}_{part}.txt");
        let actual = f(&filename);
        assert_eq!(actual, expected);
    }
}

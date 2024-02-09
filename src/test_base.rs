#[cfg(test)]
pub fn get_test_file_name(year: u32, number: u32, part: &str) -> String {
    format!("..\\advent_app_data\\{year}_{number:0>2}_{part}.txt")
}

pub fn get_test_file_name(year: u32, number: u32, part: &str, input: bool) -> String {
    let postfix = if input { "input" } else { "sample" };
    format!("..\\advent_app_data\\{year}_{number:0>2}_{part}_{postfix}.txt")
}

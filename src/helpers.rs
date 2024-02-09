pub fn read_lines(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub trait IteratorExt<T> {
    fn first(&mut self) -> T;
}

impl<T, I> IteratorExt<T> for I
where
    I: Iterator<Item = T>
{
    fn first(&mut self) -> T {
        self.next().unwrap()
    }
}

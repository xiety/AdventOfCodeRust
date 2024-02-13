use std::str::FromStr;
use std::fmt::Debug;

pub fn read_lines(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub trait IteratorExt<T> {
    fn first(&mut self) -> T;
    fn index_of(&mut self, value: T) -> usize where T: std::cmp::PartialEq;
}

impl<T, I> IteratorExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn first(&mut self) -> T {
        self.next().unwrap()
    }

    fn index_of(&mut self, value: T) -> usize where T: std::cmp::PartialEq {
        self.position(|x| x == value).unwrap()
    }
}

pub trait CapturesExt<'h> {
    fn get_type<T: FromStr>(&self, name: &str) -> T where <T as FromStr>::Err: Debug;
    fn get_str(&self, name: &str) -> &str;
}

impl<'h> CapturesExt<'h> for regex_lite::Captures<'h>
{
    fn get_type<T: FromStr>(&self, name: &str) -> T where <T as FromStr>::Err: Debug {
        let v = self.get_str(name);
        let a = v.parse::<T>();
        a.unwrap()
    }

    fn get_str(&self, name: &str) -> &str {
        self.name(name).unwrap().as_str()
    }
}

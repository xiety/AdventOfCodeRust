use std::cmp::PartialEq;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::str::FromStr;

pub fn read_lines(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_parsed<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    read_lines(filename)
        .into_iter()
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

pub trait StringExt {
    fn split_to_array<T>(&self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug;
}

impl StringExt for String {
    fn split_to_array<T>(&self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<T>().unwrap())
            .collect()
    }
}

pub trait IteratorExt<T> {
    fn first(&mut self) -> T;

    fn first_option(&mut self) -> Option<T>;

    fn index_of(&mut self, value: T) -> usize
    where
        T: PartialEq;

    fn group_by<TKey, F>(self, f: F) -> impl Iterator<Item = (TKey, Vec<T>)>
    where
        TKey: Ord,
        F: Fn(&T) -> TKey;

    fn split(self, separator: &T) -> Vec<Vec<T>>
    where
        T: PartialEq,
        T: Clone;
}

impl<T, I> IteratorExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn first(&mut self) -> T {
        self.next().unwrap()
    }

    fn first_option(&mut self) -> Option<T> {
        self.next()
    }

    fn index_of(&mut self, value: T) -> usize
    where
        T: PartialEq,
    {
        self.position(|x| x == value).unwrap()
    }

    fn group_by<TKey, F>(self, f: F) -> impl Iterator<Item = (TKey, Vec<T>)>
    where
        TKey: Ord,
        F: Fn(&T) -> TKey,
    {
        self.into_iter()
            .fold(BTreeMap::<TKey, Vec<T>>::new(), |mut acc, item| {
                let key = f(&item);
                acc.entry(key).or_default().push(item);
                acc
            })
            .into_iter()
    }

    fn split(self, separator: &T) -> Vec<Vec<T>>
    where
        T: PartialEq,
        T: Clone,
    {
        let mut result = Vec::new();
        let mut buffer = Vec::new();

        for x in self {
            if &x == separator {
                result.push(buffer.clone());
                buffer.clear();
            } else {
                buffer.push(x)
            }
        }

        if buffer.len() > 0 {
            result.push(buffer);
        }

        result
    }
}

pub trait CapturesExt<'h> {
    fn get_type<T: FromStr>(&self, name: &str) -> T
    where
        <T as FromStr>::Err: Debug;
    fn get_str(&self, name: &str) -> &str;
}

impl<'h> CapturesExt<'h> for regex_lite::Captures<'h> {
    fn get_type<T: FromStr>(&self, name: &str) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        let v = self.get_str(name);
        let a = v.parse::<T>();
        a.unwrap()
    }

    fn get_str(&self, name: &str) -> &str {
        self.name(name).unwrap().as_str()
    }
}

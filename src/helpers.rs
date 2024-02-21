use std::cmp::PartialEq;
use std::cmp::PartialOrd;
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

    fn distinct(self) -> impl Iterator<Item = T>
    where
        T: Eq + std::hash::Hash;

    fn order(self) -> impl Iterator<Item = T>
    where
        T: Ord;
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
        self.fold(BTreeMap::<TKey, Vec<T>>::new(), |mut acc, item| {
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

    fn distinct(self) -> impl Iterator<Item = T>
    where
        T: Eq + std::hash::Hash,
    {
        self.collect::<std::collections::HashSet<T>>().into_iter()
    }

    fn order(self) -> impl Iterator<Item = T>
    where
        T: Ord,
    {
        let mut v = self.collect::<Vec<T>>();
        v.sort();

        v.into_iter()
    }
}

pub trait IteratorExtClone<T>: Iterator<Item = T> + Clone {
    fn pairs(self) -> impl Iterator<Item = (T, T)>;
    fn pairs_every(self) -> impl Iterator<Item = (T, T)>;
}

impl<T, I> IteratorExtClone<T> for I
where
    I: Iterator<Item = T> + Clone,
{
    fn pairs(self) -> impl Iterator<Item = (T, T)> {
        self.pairs_every().step_by(2)
    }

    fn pairs_every(self) -> impl Iterator<Item = (T, T)> {
        self.clone().zip(self.skip(1))
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

pub fn is_intersect<T: PartialOrd>(from_a: T, to_a: T, from_b: T, to_b: T) -> bool {
    !((from_a > to_b) || (to_a < from_b))
}

pub fn intersect<T: PartialOrd>(from_a: T, to_a: T, from_b: T, to_b: T) -> Option<(T, T)> {
    if from_a >= from_b && to_a <= to_b {
        Some((from_a, to_a))
    } else if from_a <= from_b && to_a >= to_b {
        Some((from_b, to_b))
    } else if from_a <= from_b && to_a <= to_b {
        Some((from_b, to_a))
    } else if from_a >= from_b && to_a >= to_b {
        Some((from_a, to_b))
    } else {
        None
    }
}

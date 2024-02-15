use std::iter::Enumerate;

use crate::helpers::read_lines;

pub struct LinesMap {
    lines: Vec<String>,
}

impl LinesMap {
    pub fn for2d(&self, fx: i32, tx: i32, fy: i32, ty: i32) -> impl Iterator<Item = (i32, i32)> {
        let (width, height) = self.get_size();
        (fy..ty)
            .flat_map(move |y| (fx..tx).map(move |x| (x, y)))
            .filter(move |(x, y)| x >= &0 && x < &width && y >= &0 && y < &height)
    }

    pub fn chars2d(&self, fx: i32, tx: i32, fy: i32, ty: i32) -> impl Iterator<Item = char> + '_ {
        self.for2d(fx, tx, fy, ty)
            .map(move |(x, y)| self.get_char(x, y))
    }

    pub fn enumerate_chars2d(&self, fx: i32, tx: i32, fy: i32, ty: i32) -> impl Iterator<Item = (char, i32, i32)> + '_ {
        self.for2d(fx, tx, fy, ty)
            .map(move |(x, y)| (self.get_char(x, y), x, y))
    }

    pub fn load(filename: &str) -> LinesMap {
        LinesMap {
            lines: read_lines(filename),
        }
    }

    pub fn get_size(&self) -> (i32, i32) {
        let width = self.lines[0].len() as i32;
        let height = self.lines.len() as i32;
        (width, height)
    }

    pub fn get_char(&self, x: i32, y: i32) -> char {
        self.lines[y as usize].chars().nth(x as usize).unwrap()
    }

    pub fn enumerate_lines(&self) -> Enumerate<std::slice::Iter<String>> {
        self.lines.iter().enumerate()
    }
}

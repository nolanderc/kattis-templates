use std::{
    io::{stdin, BufRead},
    str::FromStr,
    vec::IntoIter,
};

#[macro_export]
macro_rules! scan {
    ($scanner:ident, $($type:ty),+) => {
        (
            $(
                $scanner.next::<$type>()
            ),+
        )
    }
}

pub struct Scanner {
    /// All lines
    lines: IntoIter<String>,
    /// Words of the current line
    words: IntoIter<String>,
}

#[allow(dead_code)]
impl Scanner {
    pub fn new() -> Scanner {
        let stdin = stdin();
        let lines = stdin
            .lock()
            .lines()
            .map(|line| line.expect("Failed to read line"))
            .collect::<Vec<String>>()
            .into_iter();

        Scanner {
            lines,
            words: Vec::new().into_iter(),
        }
    }


    /// Number of remaining lines
    pub fn remaining_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn next_line(&mut self) -> Option<String> {
        self.words = Vec::new().into_iter();
        self.lines.next()
    }

    /// Parse next word as a T
    pub fn next<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        loop {
            if let Some(word) = self.words.next() {
                break word.parse().expect("Failed to parse");
            } else {
                self.words = self
                    .lines
                    .next()
                    .expect("No more lines!")
                    .split_whitespace()
                    .map(|word| word.to_owned())
                    .collect::<Vec<_>>()
                    .into_iter();
            }
        }
    }
}


use std::{
    cell::RefCell,
    io::{stdin, Read},
    ops::{Deref, DerefMut},
    ptr,
    str::{FromStr, Lines, SplitWhitespace},
    vec::IntoIter,
};

static mut SCANNER: *mut Scanner = ptr::null_mut();

#[macro_export]
#[allow(unused_parens)]
macro_rules! scan {
    // repeat scan multiple times
    [$type:tt; $n:expr] => {{
        let n: usize = $n;
        let mut results = Vec::with_capacity(n);
        for _ in 0..n { results.push(scan!($type)); }
        results
    }};
    // repeat scan for every type on line
    [$type:ty; _] => {{
        $crate::scanner::scanner().scan_line::<$type>()
    }};
    // Get next word as string slice
    (str) => {
        $crate::scanner::scanner().next_word()
    };
    // scan into tuple
    (($($type:tt),+)) => {
        ( $( scan!($type) ),+ )
    };
    // scan single value
    ($type:ty) => {
        $crate::scanner::scanner().scan::<$type>()
    };
    // scan into tuple
    ($($type:tt),+) => {
        ( $( scan!($type) ),+ )
    };
}

pub struct Scanner {
    /// All lines
    lines: Lines<'static>,
    /// Words of the current line
    words: SplitWhitespace<'static>,
}

pub fn scanner() -> &'static mut Scanner {
    unsafe {
        if SCANNER.is_null() {
            let scanner = Box::leak(Box::new(Scanner::new()));
            SCANNER = scanner as *mut _;
        }

        SCANNER.as_mut().unwrap()
    }
}

#[allow(dead_code)]
impl Scanner {
    pub fn new() -> Scanner {
        let mut buffer = Vec::new();
        stdin().lock().read_to_end(&mut buffer).unwrap();

        // Pray to the gods that we're dealing with UTF-8
        let text = unsafe { String::from_utf8_unchecked(buffer) };
        let buffer = Box::leak(text.into_boxed_str());

        let lines = buffer.lines();

        Scanner {
            lines,
            words: "".split_whitespace(),
        }
    }

    fn consume_line(&mut self) {
        self.words = self
            .lines
            .next()
            .expect("No more lines!")
            .split_whitespace()
    }

    /// Get next word
    pub fn next_word(&mut self) -> &'static str {
        loop {
            if let Some(word) = self.words.next() {
                break word;
            } else {
                self.consume_line();
            }
        }
    }

    /// Parse next word as a T
    pub fn scan<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        loop {
            if let Some(word) = self.words.next() {
                break word.parse().expect("Failed to parse");
            } else {
                self.consume_line();
            }
        }
    }

    /// Parse next line as a Vec<T>
    pub fn scan_line<T>(&mut self) -> Vec<T>
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut data = Vec::new();

        while let Some(w) = self.words.next() {
            data.push(w.parse().unwrap());
        }

        if data.len() == 0 {
            self.consume_line();
            while let Some(w) = self.words.next() {
                data.push(w.parse().unwrap());
            }
        }

        data
    }
}




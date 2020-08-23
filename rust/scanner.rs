use std::{
    io::{stdin, Read},
    ptr,
    str::{FromStr, Lines, SplitWhitespace},
};

static mut SCANNER: *mut Scanner = ptr::null_mut();

/// Scan multiple expressions.
///
/// # Example
///
/// ```
/// scan_block! {
///     count: usize,
///     values: [f32; count],
/// }
/// println!("I have {} values: {:?}", count, values);
/// ```
#[macro_export]
#[allow(unused_parens)]
macro_rules! scan_block {
    (
        $(
            $ident:ident: $ty:tt
        ),+ $(,)?
    ) => {
        $(
            let $ident = scan_block!(@scan $ty);
        )+
    };

    (@scan [$type:tt; $n:expr]) => { scan!($type; $n) };
    (@scan $ty:tt) => { scan!($ty) };
}

/// Scan a single expression.
///
/// # Example
///
/// ```
/// let count = scan!(usize);
/// let values = scan![f32; count];
/// println!("I have {} values: {:?}", count, values);
/// ```
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
    // repeat scan for every word on line
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

/// Get the next line in the input
///
/// # Example
///
/// ```
/// let line = next_line!();
/// println!("I spy with my little eye: {:?}", line);
/// ```
macro_rules! next_line {
    () => {
        $crate::scanner::scanner().next_line()
    }
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

    /// Get the next (complete) line in the input.
    pub fn next_line(&mut self) -> &'static str {
        self.lines.next().unwrap()
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

        while data.is_empty() {
            self.consume_line();
            data = self.words.by_ref().map(|w| w.parse().unwrap()).collect()
        }

        data
    }
}

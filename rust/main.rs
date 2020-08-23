#![allow(unused_imports)]
#![allow(unused_macros)]
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    io::{self, stdin, BufRead, Read, Write},
};

#[macro_use]
mod scanner;
use self::scanner::*;

fn main() {
    scan_block! {
        // get the number of values
        count: usize,
        values: [f32; count],
    }

    println!("I have {} values: {:?}", count, values);
}


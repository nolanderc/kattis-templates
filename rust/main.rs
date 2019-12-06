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
    let n = scan!(usize);
    let xs = scan!(usize; n);
}


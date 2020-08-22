#![allow(unused_imports)]
#![allow(unused_macros)]

#[macro_use]
mod scanner;

use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    io::{stdout, BufWriter, Write},
};

fn main() {
    let n = scan!(usize);
    let xs = scan!(usize; n);
}

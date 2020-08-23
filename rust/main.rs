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
    scan_block! {
        // get the number of values
        count: usize,
        values: [f32; count],
    }

    let line = next_line!();

    println!("I have {} values: {:?}", count, values);
    println!("I spy with my little eye: {:?}", line);
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
use std::{fs::File, io::{BufReader, BufRead}};

pub fn print_solution() {
    let input = File::open("./inputs/input15.txt").unwrap();
    let lines = BufReader::new(input).lines();
}
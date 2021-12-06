use std::{fs::File, io::{BufRead, BufReader}};

pub fn print_solution() {
    let input = File::open("inputs/input1.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let vals: Vec<u32> = lines.into_iter().map(|x| x.unwrap().parse().unwrap()).collect();
    let mut increases = 0;
    let mut prev = u32::MAX;
    for &val in vals.iter() {
        if val > prev {
            increases += 1;
        }
        prev = val;
    }
    println!("Day 1 Part 1: {}", increases);
    increases = 0;
    for i in 0..vals.len() - 3 {
        if vals[i + 3] >  vals[i] {
            increases += 1;
        }
    }
    println!("Day 1 Part 2: {}", increases);
}
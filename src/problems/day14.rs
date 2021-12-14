use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub fn print_solution() {
    let input = File::open("./inputs/input14.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let template: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c as u8)
        .collect();
    let last = *template.last().unwrap();
    lines.next().unwrap().unwrap();
    let mut insertion_rules: HashMap<(u8, u8), u8> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" -> ").collect();
        let pair = (
            parts[0].chars().nth(0).unwrap() as u8,
            parts[0].chars().nth(1).unwrap() as u8,
        );
        let c = parts[1].chars().nth(0).unwrap() as u8;
        insertion_rules.insert(pair, c);
    }
    let mut pair_counts: HashMap<(u8, u8), usize> = HashMap::new();
    for (a, b) in template.iter().tuple_windows() {
        *pair_counts.entry((*a, *b)).or_default() += 1;
    }
    for i in 0..40 {
        let mut new_pair_counts = HashMap::new();
        for (pair, count) in pair_counts.iter() {
            let c = insertion_rules[pair];
            let pair1 = (pair.0, c);
            *new_pair_counts.entry(pair1).or_default() += *count;
            let pair2 = (c, pair.1);
            *new_pair_counts.entry(pair2).or_default() += *count;
        }
        pair_counts = new_pair_counts;
        if i == 9 {
            let p1 = calc_answer(&pair_counts, last);
            println!("Day 14 Part 1: {}", p1);
        }
    }
    let p2 = calc_answer(&pair_counts, last);
    println!("Day 14 Part 2: {}", p2);
}

fn calc_answer(pair_counts: &HashMap<(u8, u8), usize>, last: u8) -> usize {
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for (pair, &count) in pair_counts {
        *counts.entry(pair.0).or_default() += count;
    }
    *counts.entry(last).or_default() += 1;
    counts.values().max().unwrap() - counts.values().min().unwrap()
}
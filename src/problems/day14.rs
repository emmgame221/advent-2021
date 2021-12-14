use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("./inputs/input14.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let mut template: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c as u8)
        .collect();
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
    for _n in 0..10 {
        //println!("{}", n);
        template = do_insertions(&template, &insertion_rules);
    }
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for c in template.iter() {
        let count = if let Some(n) = counts.get(c) {
            *n + 1
        } else {
            1
        };
        counts.insert(*c, count);
    }
    let p1 = counts.values().max().unwrap() - counts.values().min().unwrap();
    println!("Day 14 Part 1: {}", p1);
    let backup = template.clone();
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for slice in 1..(template.len() - 1) {
        println!("Slice: {}", slice);
        template = backup[slice - 1..=slice].to_vec();
        for _n in 0..30 {
            //println!("{}", n + 10);
            template = do_insertions(&template, &insertion_rules);
        }
        for c in template.iter() {
            let count = if let Some(n) = counts.get(c) {
                *n + 1
            } else {
                1
            };
            counts.insert(*c, count);
        }
        if slice - 1 != 0 {
            *counts.get_mut(&template[0]).unwrap() -= 1;
        }
    }
    
    let p2 = counts.values().max().unwrap() - counts.values().min().unwrap();
    println!("Day 14 Part 2: {}", p2);
}

#[inline(always)]
fn do_insertions(s: &[u8], rules: &HashMap<(u8, u8), u8>) -> Vec<u8> {
    let mut next = Vec::new();
    for i in 0..s.len() - 1 {
        next.push(s[i]);
        if let Some(c) = rules.get(&(s[i], s[i + 1])) {
            next.push(*c);
        }
    }
    next.push(s[s.len() - 1]);
    next
}

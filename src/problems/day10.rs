use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("./inputs/input10.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let mut total = 0;
    let mut scores: Vec<i64> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let mut score = 0;
        let mut stack: Vec<char> = Vec::new();
        let mut corrupted = false;
        for c in line.chars() {
            if ")]}>".contains(c) {
                if let Some(ch) = stack.last() {
                    if matching(*ch, c) {
                        stack.pop().unwrap();
                    } else {
                        total += p1_score(c);
                        corrupted = true;
                        break;
                    }
                } else {
                    total += p1_score(c);
                    corrupted = true;
                    break;
                }
            } else {
                stack.push(c);
            }
        }
        if !corrupted {
            while !stack.is_empty() {
                let top = stack.pop().unwrap();
                score *= 5;
                score += p2_score(top);
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("Day 10 Part 1: {}", total);
    println!("Day 10 Part 2: {}", scores[(scores.len() - 1) / 2]);
}

fn matching(open: char, close: char) -> bool {
    (open == '(' && close == ')')
        || (open == '[' && close == ']')
        || (open == '{' && close == '}')
        || (open == '<' && close == '>')
}

fn p1_score(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn p2_score(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

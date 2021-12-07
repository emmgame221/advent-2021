use std::{fs::File, io::{BufReader, BufRead}};

pub fn print_solution() {
    let input = File::open("./inputs/input7.txt").unwrap();
    let line = BufReader::new(input).lines().next().unwrap().unwrap();
    let positions: Vec<isize> = line.split(',').map(|s| s.parse().unwrap()).collect();
    println!("Day 7 Part 1: {}", part_1(&positions));
    println!("Day 7 Part 2: {}", part_2(&positions));
}

fn part_1(positions: &[isize]) -> isize {
    let max = *positions.iter().max().unwrap();
    let mut best_score = isize::MAX;
    for x in 1..=max {
        let score = positions.iter().fold(0, |a, x2| a + isize::abs(x2 - x));
        if score < best_score {
            best_score = score
        }
    }
    best_score
}

fn part_2(positions: &[isize]) -> isize {
    let max = *positions.iter().max().unwrap();
    let mut best_score = isize::MAX;
    for x in 1..=max {
        let score = positions.iter().fold(0, |a, x2| {
            let range = isize::abs(x2 - x);
            let fuel = range * (range + 1) / 2;
            a + fuel
        });
        if score < best_score {
            best_score = score
        }
    }
    best_score
}
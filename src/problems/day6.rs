use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("./inputs/input6.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let line = lines.next().unwrap().unwrap();
    let fish: Vec<u8> = line.trim().split(',').map(|s| s.parse().unwrap()).collect();
    println!("Day 6 Part 1: {}", simulate(&fish, 80));
    println!("Day 6 Part 2: {}", simulate(&fish, 256));
}

fn simulate(fish: &[u8], n: usize) -> usize {
    let mut days = [0; 9];
    for f in fish {
        days[*f as usize] += 1;
    }
    for _ in 0..n {
        let new_fish = days[0];
        for i in 0..8 {
            days[i] = days[i + 1];
        }
        days[6] += new_fish;
        days[8] = new_fish;
    }
    days.iter().sum()
}

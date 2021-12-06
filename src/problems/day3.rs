use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("inputs/input3.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let mut zero_counts = vec![0; 12];
    let mut one_counts = vec![0; 12];
    let mut o2_nums: Vec<String> = Vec::new();
    let mut co2_nums: Vec<String> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        o2_nums.push(line.clone());
        co2_nums.push(line.clone());
        for i in 0..12 {
            if line.chars().nth(i).unwrap() == '0' {
                zero_counts[i] += 1;
            } else {
                one_counts[i] += 1;
            }
        }
    }
    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();
    for i in 0..12 {
        if zero_counts[i] > one_counts[i] {
            gamma_str.push('0');
            epsilon_str.push('1');
        } else {
            gamma_str.push('1');
            epsilon_str.push('0');
        }
    }
    let mut o2_one_counts = one_counts.clone();
    let mut o2_zero_counts = zero_counts.clone();
    let mut i = 0;
    while o2_nums.len() > 1 {
        println!("{}", o2_nums.len());
        if o2_zero_counts[i] > o2_one_counts[i] {
            o2_nums = o2_nums
                .iter()
                .filter(|&s| {
                    //println!("{} {}", s, i);
                    s.chars().nth(i).unwrap() == '0'
                })
                .map(|s| s.to_owned().clone())
                .collect();
        } else {
            o2_nums = o2_nums
                .iter()
                .filter(|&s| {
                    //println!("{} {}", s, i);
                    s.chars().nth(i).unwrap() == '1'
                })
                .map(|s| s.to_owned().clone())
                .collect();
        }
        o2_one_counts = vec![0; 12];
        o2_zero_counts = vec![0; 12];
        for num in o2_nums.iter() {
            for j in 0..12 {
                if num.chars().nth(j).unwrap() == '0' {
                    o2_zero_counts[j] += 1;
                } else {
                    o2_one_counts[j] += 1;
                }
            }
        }
        i += 1;
    }
    let mut co2_one_counts = one_counts.clone();
    let mut co2_zero_counts = zero_counts.clone();
    i = 0;
    while co2_nums.len() > 1 {
        if co2_zero_counts[i] > co2_one_counts[i] {
            co2_nums = co2_nums
                .iter()
                .filter(|&s| s.chars().nth(i).unwrap() == '1')
                .map(|s| s.to_owned().clone())
                .collect();
        } else {
            co2_nums = co2_nums
                .iter()
                .filter(|&s| s.chars().nth(i).unwrap() == '0')
                .map(|s| s.to_owned().clone())
                .collect();
        }
        co2_one_counts = vec![0; 12];
        co2_zero_counts = vec![0; 12];
        for num in co2_nums.iter() {
            for j in 0..12 {
                if num.chars().nth(j).unwrap() == '0' {
                    co2_zero_counts[j] += 1;
                } else {
                    co2_one_counts[j] += 1;
                }
            }
        }
        i += 1;
    }
    let gamma = i64::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = i64::from_str_radix(&epsilon_str, 2).unwrap();
    let o2 = i64::from_str_radix(&o2_nums[0], 2).unwrap();
    let co2 = i64::from_str_radix(&co2_nums[0], 2).unwrap();
    println!("Day 3 Part 1: {}", gamma * epsilon);
    println!("Day 3 Part 2: {}", o2 * co2);
}

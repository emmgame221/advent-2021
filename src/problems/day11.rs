use std::{fs::File, io::{BufReader, BufRead}};
use crate::util::adjacent_with_diags;

pub fn print_solution() {
    let input = File::open("./inputs/input11.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let mut octopi: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        octopi.push(line.chars().map(|c| c.to_string().parse().unwrap()).collect());
    }
    let mut p2_octopi = octopi.clone();
    assert_eq!(octopi.len(), 10);
    assert_eq!(octopi[0].len(), 10);
    let mut total = 0;
    for _i in 1..=100 {
        total += sim_step(&mut octopi);
        //print_octopi(&octopi);
        //println!("steps: {} total flashes: {}", i, total);
    }
    println!("Day 11 Part 1: {}", total);
    let mut step = 0;
    loop {
        step += 1;
        if sim_step_p2(&mut p2_octopi) {
            break;
        }
    }
    println!("Day 11 Part 2: {}", step);
}

fn sim_step(octopi: &mut Vec<Vec<usize>>) -> usize {
    let mut flashes = 0;
    let mut flashed = vec![vec![false; 10]; 10];
    //print_octopi(&octopi);
    for row in octopi.iter_mut() {
        for octopus in row.iter_mut() {
            *octopus += 1;
        }
    }
    let mut done = false;
    while !done{
        done = true;
        //print_octopi(&octopi);
        for y in 0..10 {
            for x in 0..10 {
                if !flashed[y][x] && octopi[y][x] > 9 {
                    flashed[y][x] = true;
                    done = false;
                    flashes += 1;
                    //println!("x:{} y:{} adj:{:?}", x, y, adjacent(x, y, 10, 10));
                    for (adj_x, adj_y) in adjacent_with_diags(x, y, 10, 10) {
                        octopi[adj_y][adj_x] += 1;
                    }
                }
            }
        }
    }
    for y in 0..10 {
        for x in 0..10 {
            if flashed[y][x] {
                octopi[y][x] = 0;
            }
        }
    }
    flashes
}

fn sim_step_p2(octopi: &mut Vec<Vec<usize>>) -> bool {
    let mut flashed = vec![vec![false; 10]; 10];
    //print_octopi(&octopi);
    for row in octopi.iter_mut() {
        for octopus in row.iter_mut() {
            *octopus += 1;
        }
    }
    let mut done = false;
    while !done{
        done = true;
        //print_octopi(&octopi);
        for y in 0..10 {
            for x in 0..10 {
                if !flashed[y][x] && octopi[y][x] > 9 {
                    flashed[y][x] = true;
                    done = false;
                    //println!("x:{} y:{} adj:{:?}", x, y, adjacent(x, y, 10, 10));
                    for (adj_x, adj_y) in adjacent_with_diags(x, y, 10, 10) {
                        octopi[adj_y][adj_x] += 1;
                    }
                }
            }
        }
    }
    for y in 0..10 {
        for x in 0..10 {
            if flashed[y][x] {
                octopi[y][x] = 0;
            }
        }
    }
    flashed.iter().all(|r| r.iter().all(|b| *b))
}

#[allow(dead_code)]
fn print_octopi(octopi: &Vec<Vec<usize>>) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{:2}", octopi[y][x]);
        }
        println!();
    }
    println!();
}
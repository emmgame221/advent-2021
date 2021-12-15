use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use priority_queue::PriorityQueue;

use crate::util::adjacent;

pub fn print_solution() {
    let input = File::open("./inputs/input15.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let row = line.trim().chars().map(|c| c.to_string().parse().unwrap()).collect();
        grid.push(row);
    }
    println!("Day 15 Part 1: {}", part1(&grid));
}

fn part1(grid: &Vec<Vec<usize>>) -> usize {
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut q: PriorityQueue<(usize, usize), usize> = PriorityQueue::new();
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            dist.insert((x, y), usize::MAX - 1000);
            q.push((x, y), usize::MAX - 1000);

        }
    }
    dist.insert((0, 0), 0);
    q.push_decrease((0, 0), 0);
    while !q.is_empty() {
        let ((x, y), d) = q.pop().unwrap();
        for v in adjacent(x, y, grid[y].len(), grid.len()) {
            let other = d + grid[v.1][v.0];
            if dist[&v] > other {
                dist.insert(v, other);
                q.push_decrease(v, other);
            }
        }
    }
    dist[&(grid[0].len() - 1, grid.len() - 1)]
}
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub fn print_solution() {
    let input = File::open("./inputs/input9.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let mut map: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let row = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        map.push(row);
    }
    let mut low_points: Vec<usize> = Vec::new();
    let mut coords: Vec<(usize, usize)> = Vec::new();
    for y in 0..map.len() {
        let row = &map[y];
        for x in 0..row.len() {
            if x == 0 {
                if y == 0 {
                    if row[x] < row[x + 1] && row[x] < map[y + 1][x] {
                        low_points.push(row[x]);
                        coords.push((x, y));
                    }
                } else if y == map.len() - 1 {
                    if row[x] < row[x + 1] && row[x] < map[y - 1][x] {
                        low_points.push(row[x]);
                        coords.push((x, y));
                    }
                } else {
                    if row[x] < row[x + 1] && row[x] < map[y + 1][x] && row[x] < map[y - 1][x] {
                        low_points.push(row[x]);
                        coords.push((x, y));
                    }
                }
            } else if x == row.len() - 1 {
                if y == 0 {
                    if row[x] < row[x - 1] && row[x] < map[y + 1][x] {
                        low_points.push(row[x]);
                        coords.push((x, y));
                    }
                } else if y == map.len() - 1 {
                    if row[x] < row[x - 1] && row[x] < map[y - 1][x] {
                        low_points.push(row[x]);
                        coords.push((x, y));
                    }
                } else {
                    if row[x] < row[x - 1] && row[x] < map[y + 1][x] && row[x] < map[y - 1][x] {
                        low_points.push(row[x]);
                        coords.push((x, y));
                    }
                }
            } else {
                if y == 0 {
                    if row[x] < row[x + 1] && row[x] < row[x - 1] && row[x] < map[y + 1][x] {
                        low_points.push(row[x]);
                        coords.push((x, y));
                    }
                } else if y == map.len() - 1 {
                    if row[x] < row[x + 1] && row[x] < row[x - 1] && row[x] < map[y - 1][x] {
                        low_points.push(row[x]);
                        coords.push((x, y));
                    }
                } else {
                    if row[x] < row[x + 1]
                        && row[x] < row[x - 1]
                        && row[x] < map[y + 1][x]
                        && row[x] < map[y - 1][x]
                    {
                        low_points.push(row[x]);
                        coords.push((x, y));
                    }
                }
            }
        }
    }
    let p1: usize = low_points.iter().map(|h| risk_level(*h)).sum();
    println!("Day 9 Part 1: {}", p1);
    let p2: usize = coords
        .iter()
        .map(|c| basin_size(&map, c.0, c.1, &mut HashMap::new()))
        .sorted()
        .rev()
        .take(3)
        .fold(1, |a, b| a * b);
    println!("Day 9 Part 2: {}", p2);
}

fn risk_level(h: usize) -> usize {
    h + 1
}

fn basin_size(
    map: &Vec<Vec<usize>>,
    x: usize,
    y: usize,
    found: &mut HashMap<(usize, usize), bool>,
) -> usize {
    found.insert((x, y), true);
    if y >= map.len() || x >= map[y].len() {
        found.insert((x, y), true);
        0
    } else if map[y][x] == 9 {
        found.insert((x, y), true);
        0
    } else {
        let right = if *found.get(&(x + 1, y)).unwrap_or(&false) {
            0
        } else {
            basin_size(map, x + 1, y, found)
        };
        let left = if *found.get(&(x - 1, y)).unwrap_or(&false) {
            0
        } else {
            basin_size(map, x - 1, y, found)
        };
        let up = if *found.get(&(x, y + 1)).unwrap_or(&false) {
            0
        } else {
            basin_size(map, x, y + 1, found)
        };
        let down = if *found.get(&(x, y - 1)).unwrap_or(&false) {
            0
        } else {
            basin_size(map, x, y - 1, found)
        };
        let val = 1 + right + left + up + down;
        val
    }
}

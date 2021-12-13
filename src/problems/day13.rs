use std::{fs::File, io::{BufReader, BufRead}};

pub fn print_solution() {
    let input = File::open("./inputs/input13.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let mut points: Vec<(isize, isize)> = Vec::new();
    let mut folds: Vec<(isize, bool)> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        if line.starts_with("fold") {
            let line = line.trim_start_matches("fold along ");
            if line.starts_with("x") {
                let x = line.trim_start_matches("x=").parse().unwrap();
                folds.push((x, true));
            } else if line.starts_with("y") {
                let y = line.trim_start_matches("y=").parse().unwrap();
                folds.push((y, false));
            } else {
                panic!("Unexpected fold along instruction");
            }
        } else if !(line == "") {
            let parts: Vec<isize> = line.trim().split(',').into_iter().map(|s| s.parse().unwrap()).collect();
            points.push((parts[0], parts[1]));
        }
    }
    points = fold(&points, folds[0].0, folds[0].1);
    println!("Day 13 Part 1: {}", points.len());
    for i in 1..folds.len() {
        points = fold(&points, folds[i].0, folds[i].1);
    }
    println!("Day 13 Part 2: ");
    print_grid(&points);
}

fn fold(points: &[(isize, isize)], fold: isize, hori: bool) -> Vec<(isize, isize)> {
    let mut newpoints = Vec::new();
    if hori {
        for (x, y) in points {
            if *x <= fold {
                newpoints.push((*x, *y));
            } else {
                let dist = x - fold;
                newpoints.push((fold - dist, *y));
            }
        }
    } else {
        for (x, y) in points {
            if *y <= fold {
                newpoints.push((*x, *y));
            } else {
                let dist = y - fold;
                newpoints.push((*x, fold - dist));
            }
        }
    }
    newpoints.sort_unstable();
    newpoints.dedup();
    newpoints
}

fn print_grid(points: &[(isize, isize)]) {
    let max_x = points.iter().max_by(|(x1, _y1), (x2, _y2)| x1.cmp(x2)).unwrap().0;
    let max_y = points.iter().max_by(|(_x1, y1), (_x2, y2)| y1.cmp(y2)).unwrap().1;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
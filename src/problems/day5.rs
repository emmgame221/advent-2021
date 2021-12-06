use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("./inputs/input5.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let line_segs: Vec<LineSeg> = lines.map(|l| LineSeg::from_str(&l.unwrap())).collect();
    println!("Day 5 Part 1: {}", part_1(&line_segs));
    println!("Day 5 Part 2: {}", part_2(&line_segs));
}

fn part_1(segs: &[LineSeg]) -> usize {
    let mut locs: HashMap<Point, i32> = HashMap::new();
    for ls in segs.iter() {
        if ls.is_hori() || ls.is_vert() {
            let points = ls.all_points();
            for point in points {
                let count = match locs.get(&point) {
                    Some(c) => *c + 1,
                    None => 1,
                };
                locs.insert(point, count);
            }
        }
    }
    locs.values().filter(|&&c| c >= 2).count()
}

fn part_2(segs: &[LineSeg]) -> usize {
    let mut locs: HashMap<Point, i32> = HashMap::new();
    for ls in segs.iter() {
        let points = ls.all_points();
        for point in points {
            let count = match locs.get(&point) {
                Some(c) => *c + 1,
                None => 1,
            };
            locs.insert(point, count);
        }
    }
    locs.values().filter(|&&c| c >= 2).count()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let pair: Vec<i32> = s
            .trim()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        Self {
            x: pair[0],
            y: pair[1],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LineSeg {
    p1: Point,
    p2: Point,
}

impl LineSeg {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.trim().split("->").collect();
        Self {
            p1: Point::from_str(parts[0]),
            p2: Point::from_str(parts[1]),
        }
    }

    fn is_hori(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_vert(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn all_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let x_step = (self.p2.x-self.p1.x).signum();
        let y_step = (self.p2.y-self.p1.y).signum();
        let mut x = self.p1.x;
        let mut y = self.p1.y;
        while !(x == self.p2.x && y == self.p2.y) {
            points.push(Point { x, y });
            x += x_step;
            y += y_step;
        }
        points.push(self.p2);
        points
    }
}

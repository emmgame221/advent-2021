use std::{fs::File, io::{BufReader, BufRead}, convert::{TryFrom, TryInto}};

pub fn print_solution() {
    let input = File::open("inputs/input2.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let dirs: Vec<Direction> = lines.into_iter().map(|x| x.unwrap().try_into().unwrap()).collect();
    let (hor, depth) = part_one(&dirs);
    println!("Day 2 Part 1: {}", hor * depth);
    let (hor, depth) = part_two(&dirs);
    println!("Day 2 Part 2: {}", hor * depth);
}

fn part_one(dirs: &[Direction]) -> (i32, i32) {
    let mut hor = 0;
    let mut depth = 0;
    for dir in dirs {
        match dir {
            Direction::Forward(x) => hor += x,
            Direction::Down(x) => depth += x,
            Direction::Up(x) => depth -= x,
        }
    }
    (hor, depth)
}

fn part_two(dirs: &[Direction]) -> (i32, i32) {
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;
    for dir in dirs {
        match dir {
            Direction::Forward(x) => {
                hor += x;
                depth += x * aim;
            }
            Direction::Down(x) => aim += x,
            Direction::Up(x) => aim -= x,
        }
    }
    (hor, depth)
}

#[derive(Debug)]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl TryFrom<String> for Direction {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split_ascii_whitespace().collect();
        let err_msg = "A direction can only be created from a string consisting of a direction(forward, up, or down) and a positive integer.";
        if parts.len() != 2 {
            return Err(err_msg);
        }
        let val  = parts[1].parse::<i32>();
        if val.is_err() {
            return Err(err_msg);
        }
        let val = val.unwrap();
        match parts[0] {
            "forward" => Ok(Direction::Forward(val)),
            "down" => Ok(Direction::Down(val)),
            "up" => Ok(Direction::Up(val)),
            _ => Err(err_msg)
        }
    }
}
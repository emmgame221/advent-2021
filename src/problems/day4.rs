use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn print_solution() {
    let input = File::open("./inputs/input4.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let first = lines.next().unwrap().unwrap();
    let draws: Vec<u16> = first
        .trim()
        .split(',')
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut board_str = String::new();
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            if !board_str.is_empty() {
                boards.push(BingoBoard::from_str(&board_str).unwrap());
                board_str.clear();
            }
        } else {
            board_str.push_str(&line);
            board_str.push('\n');
        }
    }
    let mut found_first = false;
    let mut bingo_count = 0;
    for num in draws {
        for board in boards.iter_mut() {
            if !board.bingo_found {
                board.mark_num(num);
                if board.is_bingo() {
                    if !found_first {
                        println!("Day 4 Part 1: {}", board.unmarked_sum() * num);
                        found_first = true;
                    }
                    board.bingo_time = bingo_count;
                    bingo_count += 1;
                    board.bingo_num = num;
                    board.bingo_found = true;
                }
            }
        }
    }
    let mut last_found = -1;
    let mut score = 0;
    for board in boards.iter() {
        if board.bingo_time > last_found {
            last_found = board.bingo_time;
            score = board.bingo_num * board.unmarked_sum();
        }
    }
    println!("Day 4 Part 2: {}", score);
}

struct BingoBoard {
    board: [[u16; 5]; 5],
    hits: [[bool; 5]; 5],
    bingo_found: bool,
    bingo_time: i32,
    bingo_num: u16,
}

impl FromStr for BingoBoard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.trim().split('\n').collect();
        let mut board = [[0; 5]; 5];
        if lines.len() != 5 {
            return Err("Bingo board must be 5 lines long");
        }
        for (i, &line) in lines.iter().enumerate() {
            let nums: Vec<&str> = line.trim().split_whitespace().collect();
            if nums.len() != 5 {
                return Err(
                    "Each line of a bingo board must have 5 values separated by a single space",
                );
            }
            for j in 0..5 {
                if let Ok(num) = nums[j].parse() {
                    board[i][j] = num;
                } else {
                    return Err("Each value in a bingo board line must be a positive integer");
                }
            }
        }
        let hits = [[false; 5]; 5];
        Ok(Self {
            board,
            hits,
            bingo_found: false,
            bingo_time: -1,
            bingo_num: 0,
        })
    }
}

impl BingoBoard {
    fn mark_num(&mut self, num: u16) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == num {
                    self.hits[i][j] = true;
                    return;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        for row in 0..5 {
            if self.check_horizontal(row) {
                return true;
            }
        }
        for col in 0..5 {
            if self.check_vertical(col) {
                return true;
            }
        }
        false
    }

    fn check_horizontal(&self, row: usize) -> bool {
        self.hits[row][0]
            && self.hits[row][1]
            && self.hits[row][2]
            && self.hits[row][3]
            && self.hits[row][4]
    }

    fn check_vertical(&self, col: usize) -> bool {
        self.hits[0][col]
            && self.hits[1][col]
            && self.hits[2][col]
            && self.hits[3][col]
            && self.hits[4][col]
    }

    fn unmarked_sum(&self) -> u16 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.hits[i][j] {
                    sum += self.board[i][j];
                }
            }
        }
        sum
    }
}

use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("./inputs/input8.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let mut entries: Vec<(String, String)> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('|').collect();
        let inputs = parts[0].trim().to_string();
        let outputs = parts[1].trim().to_string();
        entries.push((inputs, outputs));
    }
    let mut count = 0;
    for entry in entries.iter() {
        for output in entry.1.split(' ') {
            match output.len() {
                2 => count += 1,
                3 => count += 1,
                4 => count += 1,
                7 => count += 1,
                _ => {}
            }
        }
    }
    println!("Day 8 Part 1: {}", count);
    println!("Day 8 Part 2: {}", part_2(&entries));
}

fn part_2(entries: &[(String, String)]) -> i64 {
    let mut total = 0;
    for entry in entries.iter() {
        let outputs: Vec<&str> = entry.1.split(' ').collect();
        let sigs = "abcdefg"
            .chars()
            .permutations(7)
            .filter(|p| is_satisfying_assignment(entry, p))
            .next()
            .unwrap();
        let mut output_str = String::new();
        for output in outputs.iter() {
            match output.len() {
                2 => output_str.push('1'),
                3 => output_str.push('7'),
                4 => output_str.push('4'),
                5 => {
                    let e = sigs[4];
                    let c = sigs[2];
                    if output.contains(e) {
                        output_str.push('2');
                    } else if output.contains(c) {
                        output_str.push('3');
                    } else {
                        output_str.push('5');
                    }
                }
                6 => {
                    let d = sigs[3];
                    let e = sigs[4];
                    if output.contains(d) {
                        if output.contains(e) {
                            output_str.push('6');
                        } else {
                            output_str.push('9');
                        }
                    } else {
                        output_str.push('0');
                    }
                }
                7 => output_str.push('8'),
                _ => panic!("Invalid length output string"),
            }
        }
        //println!("{:?} {}", outputs, output_str);
        total += output_str.parse::<i64>().unwrap();
    }
    total
}

fn is_satisfying_assignment(entry: &(String, String), abcdefg: &[char]) -> bool {
    let a = abcdefg[0];
    let b = abcdefg[1];
    let c = abcdefg[2];
    let d = abcdefg[3];
    let e = abcdefg[4];
    let f = abcdefg[5];
    let g = abcdefg[6];
    let mut res = true;
    //println!("{:?} {:?}", entry, abcdefg);
    for input in entry.0.split(' ') {
        match input.len() {
            2 => {
                if !(input.contains(c) && input.contains(f)) {
                    res = false;
                }
            }
            3 => {
                if !(input.contains(a) && input.contains(c) && input.contains(f)) {
                    res = false;
                }
            }
            4 => {
                if !(input.contains(b)
                    && input.contains(c)
                    && input.contains(d)
                    && input.contains(f))
                {
                    res = false
                }
            }
            5 => {
                if !(input.contains(a)
                    && input.contains(c)
                    && input.contains(d)
                    && input.contains(e)
                    && input.contains(g))
                    && !(input.contains(a)
                        && input.contains(c)
                        && input.contains(d)
                        && input.contains(f)
                        && input.contains(g))
                    && !(input.contains(a)
                        && input.contains(b)
                        && input.contains(d)
                        && input.contains(f)
                        && input.contains(g))
                {
                    res = false
                }
            }
            6 => {
                if !(input.contains(a)
                    && input.contains(b)
                    && input.contains(c)
                    && input.contains(e)
                    && input.contains(f)
                    && input.contains(g))
                    && !(input.contains(a)
                        && input.contains(b)
                        && input.contains(d)
                        && input.contains(e)
                        && input.contains(f)
                        && input.contains(g))
                    && !(input.contains(a)
                        && input.contains(b)
                        && input.contains(c)
                        && input.contains(d)
                        && input.contains(f)
                        && input.contains(g))
                {
                    res = false
                }
            }
            7 => {
                // All assignments satisfy
            }
            _ => {
                panic!("Invalid length")
            }
        }
        //println!("{} satisfies?:{}", input, res);
    }
    res
}

#[cfg(test)]
mod test {
    use super::is_satisfying_assignment;

    #[test]
    fn test_satisfying_1() {
        let inputs = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab".to_string();
        let outputs = "cdfeb fcadb cdfeb cdbaf".to_string();
        assert!(is_satisfying_assignment(
            &(inputs, outputs),
            &['d', 'e', 'a', 'f', 'g', 'b', 'c']
        ));
    }
}

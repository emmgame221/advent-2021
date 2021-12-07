use clap::{App, Arg};

mod problems;

use crate::problems::*;

fn main() {
    let matches = App::new("Advent of Code 2021")
                                .arg(Arg::with_name("problems")
                                    .short("p")
                                    .long("problems")
                                    .takes_value(true)
                                    .default_value("all")
                                    .use_delimiter(true))
                                .get_matches();
    let values: Vec<&str> = matches.values_of("problems").unwrap().collect();
    if values.contains(&"all") {
        run_all();
    } else {
        for value in values {
            match value {
                "1" => {
                    day1::print_solution();
                }
                "2" => {
                    day2::print_solution();
                }
                "3" => {
                    day3::print_solution();
                }
                "4" => {
                    day4::print_solution();
                }
                "5" => {
                    day5::print_solution();
                }
                "6" => {
                    day6::print_solution();
                }
                "7" => {
                    day7::print_solution();
                }
                /*"8" => {
                    day8::print_solution();
                }
                "9" => {
                    day9::print_solution();
                }
                "10" => {
                    day10::print_solution();
                }
                "11" => {
                    day11::print_solution();
                }*/
                _ => {}
            }
        }
    }
}

fn run_all() {
    day1::print_solution();
    day2::print_solution();
    day3::print_solution();
    day4::print_solution();
    day5::print_solution();
    day6::print_solution();
    day7::print_solution();
    /*day8::print_solution();
    day9::print_solution();
    day10::print_solution();
    day11::print_solution();*/
}
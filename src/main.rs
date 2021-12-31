use std::{env, fs, process};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("A day is required!");
        process::exit(1);
    }
    let day = &args[1];
    let input = read_input(day);

    println!("Calculating solution for day: {}", day);

    let (part_one, part_two) = solve_fn(day)(&input);

    println!("The solution to Part One is: {}", part_one);
    println!("The solution to Part Two is: {}", part_two);
}

fn read_input(day: &String) -> Vec<String> {
    let filename = format!("inputs/day{:0>2}", day).to_string();
    println!("Reading input from file: {}", filename);

    match fs::read_to_string(filename) {
        Ok(contents) => contents.lines().map(String::from).collect(),
        Err(e) => {
            println!("Could not read file because: {}", e);
            process::exit(1);
        }
    }
}

fn solve_fn<T: AsRef<str>>(day: &String) -> fn(&[T]) -> (usize, usize) {
    match day.as_str() {
        "1" => day01::solve,
        "2" => day02::solve,
        "3" => day03::solve,
        "4" => day04::solve,
        "5" => day05::solve,
        "6" => day06::solve,
        "7" => day07::solve,
        "8" => day08::solve,
        _ => {
            println!("No implementation found for day {}!", day);
            process::exit(1);
        }
    }
}

use std::error::Error;
use std::io::{self, Write};
use std::process::Command;
use std::{env, process};

#[path = "days/day1/day1.rs"]
mod day1;

#[path = "days/day2/day2.rs"]
mod day2;

#[path = "days/day3/day3.rs"]
mod day3;

#[path = "days/day4/day4.rs"]
mod day4;

#[path = "days/day5/day5.rs"]
mod day5;

#[path = "days/day7/day7.rs"]
mod day7;

#[path = "days/day8/day8.rs"]
mod day8;

mod file_reader;
mod helper;

enum WhichPuzzle {
    First,
    Second,
}

fn print_usage() {
    println!("Usage: aoc <puzzle> [t[est]]");
    println!("  <puzzle>    which puzzle to run, e.g., 1.1, 1.2, 2.1, etc.");
    println!("  test        use example input instead of real input");
    println!("  t           alias for test");
    println!("Or: aoc i <day>");
    println!("  i           download input for day");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    // If the first arg is i, download the input and exit
    if &args[1] == "i" {
        download_input(&args[2]);
        return;
    }

    let puzzle_number = &args[1];

    let mut use_test_input = false;
    if args.len() > 2 {
        use_test_input = ["test", "t"].contains(&args[2].as_str())
    }

    let (puzzle_major, puzzle_minor) = match parse_puzzle_option(puzzle_number) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    let answer = match puzzle_major {
        "1" => day1::run(puzzle_minor, use_test_input),
        "2" => day2::run(puzzle_minor, use_test_input),
        "3" => day3::run(puzzle_minor, use_test_input),
        "4" => day4::run(puzzle_minor, use_test_input),
        "5" => day5::run(puzzle_minor, use_test_input),
        "7" => day7::run(puzzle_minor, use_test_input),
        "8" => day8::run(puzzle_minor, use_test_input),
        _ => {
            eprintln!("Puzzle not found");
            process::exit(1);
        }
    };

    println!("{answer}");
}

fn parse_puzzle_option<'a>(arg: &'a str) -> Result<(&'a str, WhichPuzzle), Box<dyn Error>> {
    let puzzle_number = arg.split(".").collect::<Vec<_>>();

    if puzzle_number.len() != 2 {
        return Err("Invalid puzzle number".into());
    }

    let which_sub_puzzle = match puzzle_number[1] {
        "1" => WhichPuzzle::First,
        "2" => WhichPuzzle::Second,
        _ => {
            return Err(format!(
                "Invalid option in puzzle sub-number: {}",
                puzzle_number[1]
            ))?;
        }
    };

    Ok((puzzle_number[0], which_sub_puzzle))
}

fn download_input(day: &str) {
    let output = Command::new("./src/curl.sh")
        .arg(day)
        .output()
        .expect("Failed to execute command");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

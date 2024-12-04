use crate::{file_reader, helper::string_to_i32, WhichPuzzle};
use regex::Regex;

#[cfg(test)]
mod tests;

const INPUT_FILE_PATH: &str = "src/days/day3/input3.txt";
const TEST_INPUT_1: &str =
    r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
const TEST_INPUT_2: &str =
    r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

pub fn run(which: WhichPuzzle, use_test_input: bool) -> i32 {
    let input;
    if use_test_input {
        input = match which {
            WhichPuzzle::First => String::from(TEST_INPUT_1),
            WhichPuzzle::Second => String::from(TEST_INPUT_2),
        }
    } else {
        input = file_reader::read_file(INPUT_FILE_PATH);
    }

    match which {
        WhichPuzzle::First => run_one(&input),
        WhichPuzzle::Second => run_two(&input),
    }
}

fn run_one(input: &str) -> i32 {
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for (_, [left_operand, right_operand]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += string_to_i32(left_operand) * string_to_i32(right_operand);
    }

    sum
}

fn run_two(input: &str) -> i32 {
    let mut sum = 0;

    // Split on `don't()` to find all sections where `mul()` begins as disabled.
    let mut sections = input.split("don't()").collect::<Vec<_>>();

    // If the first word of the input is not "don't()", then the first section is the only
    // one where it starts with `mul()` enabled. Calculate any muls in it and then remove it
    // from the vec.
    let first_index_of_dont = input.find("don't()");
    if first_index_of_dont.unwrap_or(0) > 1 {
        let first_section = sections.get(0).expect("First element did not exist");
        sum += run_one(first_section);
        sections.remove(0);
    }

    // In each section where mul() is disabled, find the first do(),
    // and use everything after it as a "mul enabled" section.
    for section in &sections {
        match section.split_once("do()") {
            Some((_, on_section)) => sum += run_one(&on_section),
            _ => {}
        }
    }

    sum
}

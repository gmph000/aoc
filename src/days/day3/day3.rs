use crate::{file_reader, helper::string_to_i32, WhichPuzzle};
use regex::Regex;

#[cfg(test)]
mod tests;

const INPUT_FILE_PATH: &str = "src/days/day3/input3.txt";
const TEST_INPUT: &str =
    r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

pub fn run(which: WhichPuzzle, use_test_input: bool) -> i32 {
    let input;
    if use_test_input {
        input = String::from(TEST_INPUT);
    } else {
        input = file_reader::read_file(INPUT_FILE_PATH);
    }

    match which {
        WhichPuzzle::First => run_one(&input),
        WhichPuzzle::Second => run_two(&input),
    }
}

fn run_one(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut muls: Vec<(i32, i32)> = vec![];
    for (_, [left_operand, right_operand]) in re.captures_iter(input).map(|c| c.extract()) {
        muls.push((string_to_i32(left_operand), string_to_i32(right_operand)));
    }

    let mut sum = 0;

    for pair in muls {
        sum += pair.0 * pair.1;
    }

    sum
}

fn run_two(_input: &str) -> i32 {
    0
}

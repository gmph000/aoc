use crate::{file_reader, WhichPuzzle};

#[cfg(test)]
mod tests;

const INPUT_FILE_PATH: &str = "src/days/day2/input2.txt";
const TEST_INPUT: &str = r#""#;

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

fn run_one(_input: &str) -> i32 {
    0
}

fn run_two(_input: &str) -> i32 {
    0
}

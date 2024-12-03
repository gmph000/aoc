use crate::{file_reader, helper::string_to_i32, WhichPuzzle};

#[cfg(test)]
mod tests;

const INPUT_FILE_PATH: &str = "src/days/day2/input2.txt";
const TEST_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

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
    let mut safety_list: Vec<bool> = vec![];

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| string_to_i32(num))
            .collect::<Vec<_>>();

        let mut safe = true;
        let mut asc: Option<bool> = None;
        for i in 0..(nums.len() - 1) {
            let diff = nums[i + 1] - nums[i];

            // Check if the difference is between 1 and 3.
            if diff.abs() < 1 || diff.abs() > 3 {
                safe = false;
                break;
            }

            // Check if the value is consistently ascending or descending.
            if diff > 0 {
                if asc.is_none() {
                    asc = Some(true);
                } else if asc.unwrap() == false {
                    safe = false;
                    break;
                }
            } else if diff < 0 {
                if asc.is_none() {
                    asc = Some(false);
                } else if asc.unwrap() == true {
                    safe = false;
                    break;
                }
            }
        }

        safety_list.push(safe);
    }

    safety_list
        .iter()
        .filter(|x| **x == true)
        .count()
        .try_into()
        .expect("Could not convert isize to i32")
}

fn run_two(_input: &str) -> i32 {
    0
}

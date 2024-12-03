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

enum Direction {
    Asc,
    Desc,
    Unset,
}

// Boilerplate to decide whether to run part one or two.
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

// Part one solution.
fn run_one(input: &str) -> i32 {
    let mut safe_count = 0;

    for line in input.lines() {
        let nums: Vec<i32> = convert_line_to_nums_vector(&line);

        if check_if_safe(&nums) {
            safe_count += 1;
        }
    }

    safe_count
}

// Part two solution.
fn run_two(input: &str) -> i32 {
    let mut safe_count = 0;

    for line in input.lines() {
        let nums: Vec<i32> = convert_line_to_nums_vector(&line);

        if check_if_safe(&nums) {
            safe_count += 1;
        } else {
            if check_if_safe_after_removing_one(&nums) {
                safe_count += 1;
            }
        }
    }

    safe_count
}

/// Convert `"1 2 4 7"` to `vec![1, 2, 4, 7]`.
fn convert_line_to_nums_vector(nums: &str) -> Vec<i32> {
    nums.split_whitespace()
        .map(|num| string_to_i32(num))
        .collect::<Vec<_>>()
}

/// Checks the array based on the rules, and returns `true` if safe, `false` if unsafe.
fn check_if_safe(nums: &Vec<i32>) -> bool {
    let mut dir = Direction::Unset;

    // Compare each number to the number in front of it.
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];

        // Check if the difference is between 1 and 3.
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // Check if the value is consistently ascending or descending.
        if diff > 0 {
            match dir {
                Direction::Desc => return false,
                Direction::Unset => dir = Direction::Asc,
                _ => {}
            }
        } else if diff < 0 {
            match dir {
                Direction::Asc => return false,
                Direction::Unset => dir = Direction::Desc,
                _ => {}
            }
        }
    }

    true
}

/// Naively try removing one item at a time until the sequence is safe,
/// or give up after trying all possibilities.
fn check_if_safe_after_removing_one(nums: &Vec<i32>) -> bool {
    for i in 0..nums.len() {
        let mut nums_filtered = nums.clone();
        nums_filtered.remove(i);

        if check_if_safe(&nums_filtered) {
            return true;
        }
    }

    false
}

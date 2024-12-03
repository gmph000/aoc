use crate::{file_reader, helper::string_to_i32, WhichPuzzle};
use std::sync::atomic::{AtomicUsize, Ordering};

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

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

// For performance tracking, record how many times a function is called by incrementing
// this static counter.
fn increment_call_count() {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
}

// Boilerplate to decide whether to run part one or two and which input to use.
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

        if check_if_safe(&nums).0 {
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

        let (safe, problem_index) = check_if_safe(&nums);

        if safe {
            safe_count += 1;
        } else {
            if check_if_safe_after_removing_one(&nums, problem_index) {
                safe_count += 1;
            }
        }
    }

    // println!("called {}", CALL_COUNT.load(Ordering::SeqCst));

    safe_count
}

/// Convert `"1 2 4 7"` to `vec![1, 2, 4, 7]`.
fn convert_line_to_nums_vector(nums: &str) -> Vec<i32> {
    nums.split_whitespace()
        .map(|num| string_to_i32(num))
        .collect::<Vec<_>>()
}

/// Checks the array based on the rules, and returns `(true, 0)` if safe, `(false, problem_index)` if unsafe.
/// The `problem_index` is the index where the safety check did not pass.
fn check_if_safe(nums: &Vec<i32>) -> (bool, usize) {
    increment_call_count();
    let mut dir = Direction::Unset;

    // Compare each number to the number in front of it.
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];

        // Check if the difference is between 1 and 3.
        if diff.abs() < 1 || diff.abs() > 3 {
            return (false, i);
        }

        // Check if the value is consistently ascending or descending.
        if diff > 0 {
            match dir {
                Direction::Desc => return (false, i),
                Direction::Unset => dir = Direction::Asc,
                _ => {}
            }
        } else if diff < 0 {
            match dir {
                Direction::Asc => return (false, i),
                Direction::Unset => dir = Direction::Desc,
                _ => {}
            }
        }
    }

    (true, 0)
}

/// Remove the indexes before, at, and after the problem_index and return true if
/// one fixes the problem. Otherwise return false.
fn check_if_safe_after_removing_one(nums: &Vec<i32>, problem_index: usize) -> bool {
    // Start at problem_index - 1 unless it's already at 0.
    let starting_index = if problem_index == 0 {
        0
    } else {
        problem_index - 1
    };

    for i in starting_index..=problem_index + 1 {
        if i >= nums.len() {
            println!("Out of range");
            continue;
        }

        let mut nums_filtered = nums.clone();
        nums_filtered.remove(i);

        if check_if_safe(&nums_filtered).0 {
            return true;
        }
    }

    false
}

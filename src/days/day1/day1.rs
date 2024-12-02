use crate::{file_reader, helper::*, WhichPuzzle};

#[cfg(test)]
mod tests;

const INPUT_FILE_PATH: &str = "src/days/day1/input1.txt";
const TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

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
    let (mut left, mut right) = get_both_lists(input);

    left.sort_unstable();
    right.sort_unstable();

    let mut sum = 0;

    for i in 0..left.len() {
        let diff = (left[i] - right[i]).abs();
        sum += diff;
    }

    sum
}

fn run_two(input: &str) -> i32 {
    let (left, right) = get_both_lists(input);

    let mut sum = 0;
    let right_counts = map_occurrences(&right);

    for num in left {
        let count = right_counts.get(&num).unwrap_or(&0);

        sum += num * count;
    }

    sum
}

fn get_both_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let values = line.split_whitespace().collect::<Vec<_>>();

        assert_eq!(values.len(), 2, "More than 2 values on one line.");

        left.push(string_to_i32(values[0]));
        right.push(string_to_i32(values[1]));
    }

    (left, right)
}

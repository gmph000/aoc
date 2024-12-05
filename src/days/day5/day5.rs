use crate::{file_reader, helper::string_to_i32, WhichPuzzle};

#[cfg(test)]
mod tests;

const INPUT_FILE_PATH: &str = "src/days/day5/input5.txt";
const TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

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
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut sum = 0;

    for line in updates.lines().collect::<Vec<_>>() {
        let nums = line.split(",").collect::<Vec<_>>();
        let mut correct = true;

        for rule in rules.lines().collect::<Vec<_>>() {
            let (before, after) = rule.split_once("|").expect("Pipe split failed");

            let before_index = nums.iter().position(|x| *x == before);
            let after_index = nums.iter().position(|x| *x == after);

            match (before_index, after_index) {
                (Some(before_index), Some(after_index)) => {
                    if before_index > after_index {
                        correct = false;
                        break;
                    }
                }
                _ => {}
            }
        }

        if correct {
            sum += find_middle_number(&nums);
        }
    }

    sum
}

// Part two solution.
fn run_two(input: &str) -> i32 {
    0
}

fn find_middle_number(nums: &Vec<&str>) -> i32 {
    return string_to_i32(nums.get(nums.len() / 2).expect("bad index"));
}

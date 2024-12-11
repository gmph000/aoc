use crate::{
    file_reader,
    helper::{get_permutations, string_to_i128},
    WhichPuzzle,
};

#[cfg(test)]
mod tests;

const INPUT_FILE_PATH: &str = "src/days/day7/input7.txt";
const TEST_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

// Boilerplate to decide whether to run part one or two and which input to use.
pub fn run(which: WhichPuzzle, use_test_input: bool) -> i32 {
    let input;
    if use_test_input {
        input = String::from(TEST_INPUT);
    } else {
        input = file_reader::read_file(INPUT_FILE_PATH);
    }

    let answer = match which {
        WhichPuzzle::First => run_one(&input),
        WhichPuzzle::Second => run_two(&input),
    };

    // The answer is too big for the i32 return type, but I don't want to
    // refactor the main function.
    println!("{}", answer);

    0
}

// Part one solution.
fn run_one(input: &str) -> i128 {
    let mut total = 0;
    for line in input.lines() {
        let (expected_sum, rest) = line.split_once(": ").unwrap();
        let expected_sum = string_to_i128(expected_sum).into();
        let values = rest
            .split_whitespace()
            .map(|x| string_to_i128(x))
            .collect::<Vec<_>>();

        let number_of_values = values.len();

        for perms in get_permutations(vec!["+", "*"], number_of_values - 1) {
            let mut sum: i128 = values[0].into();
            let mut i = 1;
            for operator in perms {
                match operator {
                    "+" => sum = sum + values[i],
                    "*" => sum = sum * values[i],
                    _ => {}
                }

                i += 1;
            }

            if sum == expected_sum {
                total += sum;
                break;
            }
        }
    }

    total
}

// Part two solution.
fn run_two(input: &str) -> i128 {
    let mut total = 0;
    for line in input.lines() {
        let (expected_sum, rest) = line.split_once(": ").unwrap();
        let expected_sum = string_to_i128(expected_sum).into();
        let values = rest
            .split_whitespace()
            .map(|x| string_to_i128(x))
            .collect::<Vec<_>>();

        let number_of_values = values.len();

        for perms in get_permutations(vec!["+", "*", "||"], number_of_values - 1) {
            let mut sum: i128 = values[0].into();
            let mut i = 1;
            for operator in perms {
                match operator {
                    "+" => sum = sum + values[i],
                    "*" => sum = sum * values[i],
                    "||" => sum = string_to_i128(&format!("{}{}", sum, values[i])),
                    _ => {}
                }

                i += 1;
            }

            if sum == expected_sum {
                total += sum;
                break;
            }
        }
    }

    total
}

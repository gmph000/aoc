use crate::{file_reader, WhichPuzzle};

#[cfg(test)]
mod tests;

const INPUT_FILE_PATH: &str = "src/days/day4/input4.txt";
const TEST_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

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
    let lines = input.lines().collect::<Vec<_>>();

    let mut matrix: Vec<Vec<char>> = vec![];
    for line in lines {
        matrix.push(line.chars().collect::<Vec<_>>());
    }

    let mut matches = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'X' {
                matches += traverse_all_directions(
                    &matrix,
                    i32::try_from(i).expect(""),
                    i32::try_from(j).expect(""),
                )
            }
        }
    }

    matches
}

fn traverse_all_directions(matrix: &Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let expected = "XMAS".chars().collect::<Vec<_>>();
    let directions = [
        (1, 0),
        (1, 1),
        (0, 1),
        (1, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
    ];

    let mut matches = 0;

    'dirs: for dir in directions {
        for step in 1..=3 {
            let loc = (i + (dir.0 * step), j + (dir.1 * step));

            let loc_x = match usize::try_from(loc.0) {
                Ok(val) => val,
                Err(_) => continue 'dirs,
            };
            let loc_y = match usize::try_from(loc.1) {
                Ok(val) => val,
                Err(_) => continue 'dirs,
            };

            let step = usize::try_from(step).expect("");

            if loc_x >= matrix.len() || loc_y >= matrix[0].len() {
                continue 'dirs;
            }

            let val = matrix[loc_x][loc_y];
            // println!("{val} at {loc_x},{loc_y}");
            if val != expected[step] {
                continue 'dirs;
            }
        }

        matches += 1;
    }

    matches
}

// Part two solution.
fn run_two(_input: &str) -> i32 {
    0
}

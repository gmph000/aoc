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

// Look in all directions to see if the X is the beginning of an XMAS.
// Return the number of XMASs that started at this location.
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

            if val != expected[step] {
                continue 'dirs;
            }
        }

        matches += 1;
    }

    matches
}

// Part two solution.
fn run_two(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();

    let mut matrix: Vec<Vec<char>> = vec![];
    for line in lines {
        matrix.push(line.chars().collect::<Vec<_>>());
    }

    let mut matches = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            // Find every 'A' which is always the center of an X-MAS.
            if matrix[i][j] == 'A' {
                matches += is_x_mas_center(
                    &matrix,
                    i32::try_from(i).expect(""),
                    i32::try_from(j).expect(""),
                )
            }
        }
    }

    matches
}

// If you've found an 'A', check if it has an 'S' and 'M' diagonally, but not
// the same in both line. Return 1 if it's a X-MAS, 0 if not.
fn is_x_mas_center(matrix: &Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let top_left_to_bottom_right = [(-1, 1), (1, -1)];
    let bottom_left_to_top_right = [(-1, -1), (1, 1)];

    for checks in vec![top_left_to_bottom_right, bottom_left_to_top_right] {
        let mut found_m = false;
        let mut found_s = false;

        // Remember j is like x, i is like y.
        for dir in checks {
            let loc_x = match usize::try_from(j + dir.0) {
                Ok(val) => val,
                Err(_) => return 0,
            };
            let loc_y = match usize::try_from(i + dir.1) {
                Ok(val) => val,
                Err(_) => return 0,
            };

            if loc_y >= matrix.len() || loc_x >= matrix[0].len() {
                return 0;
            }

            let val = matrix[loc_y][loc_x];

            if val == 'M' {
                found_m = true;
            } else if val == 'S' {
                found_s = true;
            }
        }

        if !found_s || !found_m {
            return 0;
        }
    }

    1
}

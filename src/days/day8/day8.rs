use crate::{file_reader, WhichPuzzle};

#[cfg(test)]
mod tests;

const INPUT_FILE_PATH: &str = "src/days/day8/input8.txt";
const TEST_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

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

#[derive(Debug)]
struct Location {
    x: i32,
    y: i32,
}

// Part one solution.
fn run_one(input: &str) -> i32 {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut antinodes_map = vec![vec![0; map[0].len()]; map.len()];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let letter = map[y][x];

            if letter != '.' {
                let matches = find_match_locs(&letter, &map, x, y);
                for m in matches {
                    let x = i32::try_from(x).expect("could not convert");
                    let y = i32::try_from(y).expect("could not convert");

                    if x < m.x || y < m.y {
                        continue;
                    }

                    let anti_x = x - m.x;
                    let anti_y = y - m.y;

                    if anti_x >= map[0].len().try_into().expect("")
                        || anti_y >= map.len().try_into().expect("")
                    {
                        continue;
                    }

                    antinodes_map[usize::try_from(anti_y).expect("")]
                        [usize::try_from(anti_x).expect("")] = 1;
                }
            }
        }
    }

    let mut total_antinodes = 0;

    for y in 0..antinodes_map.len() {
        for x in 0..antinodes_map[0].len() {
            if antinodes_map[y][x] == 1 {
                total_antinodes += 1;
            }
        }
    }

    total_antinodes
}

// Find the distance from the starting point of any matches letters in the map.
fn find_match_locs(
    letter: &char,
    map: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
) -> Vec<Location> {
    let mut locs = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == *letter && y != start_y && x != start_x {
                // Convert the values to i32 because their differences need to be negative.
                let x = i32::try_from(x).expect("Could not convert to i32");
                let start_x = i32::try_from(start_x).expect("Could not convert to i32");
                let y = i32::try_from(y).expect("Could not convert to i32");
                let start_y = i32::try_from(start_y).expect("Could not convert to i32");

                locs.push(Location {
                    x: x - start_x,
                    y: y - start_y,
                });
            }
        }
    }

    locs
}

// Part two solution.
fn run_two(input: &str) -> i32 {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut antinodes_map = vec![vec![0; map[0].len()]; map.len()];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let letter = map[y][x];

            if letter != '.' {
                antinodes_map[y][x] = 1;

                let matches = find_match_locs(&letter, &map, x, y);
                'outer: for m in matches {
                    let x = i32::try_from(x).expect("could not convert");
                    let y = i32::try_from(y).expect("could not convert");
                    let mut match_x = m.x;
                    let mut match_y = m.y;

                    loop {
                        if x < match_x || y < match_y {
                            continue 'outer;
                        }

                        let anti_x = x - match_x;
                        let anti_y = y - match_y;

                        if anti_x >= map[0].len().try_into().expect("")
                            || anti_y >= map.len().try_into().expect("")
                        {
                            continue 'outer;
                        }

                        antinodes_map[usize::try_from(anti_y).expect("")]
                            [usize::try_from(anti_x).expect("")] = 1;

                        match_x += m.x;
                        match_y += m.y;
                    }
                }
            }
        }
    }

    let mut total_antinodes = 0;

    for y in 0..antinodes_map.len() {
        for x in 0..antinodes_map[0].len() {
            if antinodes_map[y][x] == 1 {
                total_antinodes += 1;
            }
        }
    }

    total_antinodes
}

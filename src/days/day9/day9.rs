use crate::{file_reader, helper::string_to_i32, WhichPuzzle};

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Position {
    id: Option<usize>,
}

const INPUT_FILE_PATH: &str = "src/days/day9/input9.txt";
const TEST_INPUT: &str = r#"2333133121414131402"#;

// Boilerplate to decide whether to run part one or two and which input to use.
pub fn run(which: WhichPuzzle, use_test_input: bool) -> i32 {
    let input;
    if use_test_input {
        input = String::from(TEST_INPUT);
    } else {
        input = file_reader::read_file(INPUT_FILE_PATH);
    }

    let result = match which {
        WhichPuzzle::First => run_one(&input),
        WhichPuzzle::Second => run_two(&input),
    };

    println!("{result}");

    0
}

// Part one solution.
fn run_one(input: &str) -> i128 {
    let mut memory: Vec<Position> = vec![];
    let mut next_id = 0;
    let mut is_free_space = false;

    for (_, num) in input
        .trim()
        .chars()
        .map(|x| string_to_i32(&String::from(x)))
        .enumerate()
    {
        for _ in 0..num {
            if is_free_space {
                memory.push(Position { id: None });
            } else {
                memory.push(Position { id: Some(next_id) });
            }
        }

        if !is_free_space {
            next_id += 1;
        }

        is_free_space = !is_free_space
    }

    for memory_index in (0..memory.len()).rev() {
        if memory[memory_index].id.is_none() {
            continue;
        }

        for earlier_index in 0..memory_index {
            match memory[earlier_index].id {
                Some(_) => {}
                None => {
                    // println!(
                    //     "Swapping {} and {} at indexes {}, {}",
                    //     match memory[earlier_index].id {
                    //         Some(v) => format!("{v}"),
                    //         None => String::from("None"),
                    //     },
                    //     match memory[memory_index].id {
                    //         Some(v) => format!("{v}"),
                    //         None => String::from("None"),
                    //     },
                    //     earlier_index,
                    //     memory_index
                    // );
                    memory.swap(earlier_index, memory_index);
                    break;
                }
            }
        }
    }

    let mut sum = 0;

    for (index, pos) in memory.iter().enumerate() {
        match pos.id {
            Some(id) => sum += index * id,
            None => {}
        }
    }

    sum.try_into().expect("Could not convert to i128")
}

// Part two solution.
fn run_two(input: &str) -> i128 {
    0
}

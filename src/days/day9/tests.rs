use super::*;

#[test]
fn solves_day9_1() {
    assert_eq!(run_one(&String::from(TEST_INPUT)), 1928);
    assert_eq!(
        run_one(&file_reader::read_file(INPUT_FILE_PATH)),
        6463499258318
    );
}

#[test]
fn solves_day9_2() {
    assert_eq!(run_two(&String::from(TEST_INPUT)), 2858);
}

#[test]
#[ignore = "This one is very slow."]
fn solves_day7_2_real_input() {
    assert_eq!(run_two(&file_reader::read_file(INPUT_FILE_PATH)), 0);
}

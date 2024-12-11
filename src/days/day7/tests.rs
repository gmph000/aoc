use super::*;

#[test]
fn solves_day7_1() {
    assert_eq!(run_one(&String::from(TEST_INPUT)), 3749);
    assert_eq!(
        run_one(&file_reader::read_file(INPUT_FILE_PATH)),
        1038838357795
    );
}

#[test]
fn solves_day7_2() {
    assert_eq!(run_two(&String::from(TEST_INPUT)), 11387);
}

#[test]
#[ignore = "This one is very slow."]
fn solves_day7_2_real_input() {
    assert_eq!(
        run_two(&file_reader::read_file(INPUT_FILE_PATH)),
        254136560217241
    );
}

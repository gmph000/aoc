use super::*;

#[test]
fn solves_day2_1() {
    assert_eq!(run(WhichPuzzle::First, true), 18);
    assert_eq!(run(WhichPuzzle::First, false), 2493);
}

#[test]
fn solves_day2_2() {
    assert_eq!(run(WhichPuzzle::Second, true), 9);
    assert_eq!(run(WhichPuzzle::Second, false), 1890);
}

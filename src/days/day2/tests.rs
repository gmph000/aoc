use super::*;

#[test]
fn solves_day2_1() {
    assert_eq!(run(WhichPuzzle::First, true), 2);
    assert_eq!(run(WhichPuzzle::First, false), 242);
}

#[test]
fn solves_day2_2() {
    assert_eq!(run(WhichPuzzle::Second, true), 4);
    assert_eq!(run(WhichPuzzle::Second, false), 311);
}

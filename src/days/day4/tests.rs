use super::*;

#[test]
fn solves_day2_1() {
    assert_eq!(run(WhichPuzzle::First, true), 0);
    assert_eq!(run(WhichPuzzle::First, false), 0);
}

#[test]
fn solves_day2_2() {
    assert_eq!(run(WhichPuzzle::Second, true), 0);
    assert_eq!(run(WhichPuzzle::Second, false), 0);
}

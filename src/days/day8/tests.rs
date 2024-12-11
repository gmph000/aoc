use super::*;

#[test]
fn solves_day8_1() {
    assert_eq!(run(WhichPuzzle::First, true), 14);
    assert_eq!(run(WhichPuzzle::First, false), 228);
}

#[test]
fn solves_day8_2() {
    assert_eq!(run(WhichPuzzle::Second, true), 34);
    assert_eq!(run(WhichPuzzle::Second, false), 766);
}

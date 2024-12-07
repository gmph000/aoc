use super::*;

#[test]
fn solves_day5_1() {
    assert_eq!(run(WhichPuzzle::First, true), 143);
    assert_eq!(run(WhichPuzzle::First, false), 5588);
}

#[test]
fn solves_day5_2() {
    assert_eq!(run(WhichPuzzle::Second, true), 123);
    assert_eq!(run(WhichPuzzle::Second, false), 5331);
}

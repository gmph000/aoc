use super::*;

#[test]
fn solves_day3_1() {
    assert_eq!(run(WhichPuzzle::First, true), 161);
    assert_eq!(run(WhichPuzzle::First, false), 166357705);
}

#[test]
fn solves_day3_2() {
    assert_eq!(run(WhichPuzzle::Second, true), 0);
    assert_eq!(run(WhichPuzzle::Second, false), 0);
}

use super::*;

#[test]
fn solves_day1_1() {
    assert_eq!(run(WhichPuzzle::First, true), 11);
    assert_eq!(run(WhichPuzzle::First, false), 2756096);
}

#[test]
fn solves_day1_2() {
    assert_eq!(run(WhichPuzzle::Second, true), 31);
    assert_eq!(run(WhichPuzzle::Second, false), 23117829);
}

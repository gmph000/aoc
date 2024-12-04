use super::*;

#[test]
fn solves_day3_1() {
    assert_eq!(run(WhichPuzzle::First, true), 161);
    assert_eq!(run(WhichPuzzle::First, false), 166357705);

    assert_eq!(run_one(""), 0);
    assert_eq!(run_one("bogus"), 0);
}

#[test]
fn solves_day3_2() {
    assert_eq!(run(WhichPuzzle::Second, true), 48);
    assert_eq!(run(WhichPuzzle::Second, false), 88811886);
}

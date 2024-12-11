use crate::helper::*;
#[test]
fn count_occurrences_works() {
    let x = map_occurrences(&vec![1, 2, 3, 2]);
    assert_eq!(x, HashMap::from([(1, 1), (2, 2), (3, 1)]));

    let x = map_occurrences(&vec!["a", "b", "c", "b"]);
    assert_eq!(x, HashMap::from([("a", 1), ("b", 2), ("c", 1)]));
}

#[test]
fn perms() {
    let x = get_permutations(vec!["*", "+"], 1);
    assert_eq!(x, vec![vec!["*"], vec!["+"]]);

    let x = get_permutations(vec!["*", "+"], 2);
    assert_eq!(
        x,
        vec![
            vec!["*", "*"],
            vec!["*", "+"],
            vec!["+", "*"],
            vec!["+", "+"],
        ]
    );
}

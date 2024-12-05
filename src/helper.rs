use std::collections::HashMap;
use std::hash::Hash;

pub fn string_to_i32(s: &str) -> i32 {
    i32::from_str_radix(s, 10).expect(&format!("Could not convert {} to a number.", s))
}

// pub fn string_to_usize(s: &str) -> usize {
//     usize::from_str_radix(s, 10).expect(&format!("Could not convert {} to a usize.", s))
// }

#[path = "helper_tests.rs"]
#[cfg(test)]
mod helper_tests;

/// Convert a list of numbers (or anything hashable) into a map of the number to its occurences.
///
/// # Example:
/// ```
/// let x = map_occurrences(vec![1, 2, 3, 2]);
/// assert_eq!(x, HashMap::from([
///    (1, 1),
///    (2, 2),
///    (3, 1),
/// ]));
/// ```
pub fn map_occurrences<T: Eq + Clone + Hash>(numbers: &Vec<T>) -> HashMap<T, i32> {
    let mut number_count = HashMap::new();

    for num in numbers {
        number_count
            .entry(num.clone())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    number_count
}

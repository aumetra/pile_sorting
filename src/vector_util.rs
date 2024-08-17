use std::collections::HashSet;
use std::usize;

use rand::seq::SliceRandom;
use rand::thread_rng;

/// Checks that a given sequence doesn't contain gaps.
pub fn correct_sequence(input_vector: &[u8]) -> bool {
    let all_numbers: HashSet<u8, crate::OwnHasher> = (1..input_vector.len() + 1)
        .map(|number| u8::try_from(number).unwrap())
        .collect();
    all_numbers
        .iter()
        .all(|number| input_vector.contains(number))
        && input_vector
            .iter()
            .all(|number| all_numbers.contains(number))
}
/// Gives all possible orderings of that makes a valid starting pile with a given lenght.
pub fn all_sequences(lenght: usize) -> Vec<Vec<u8>> {
    let base_vector: Vec<u8> = (1u8..(u8::try_from(lenght + 1).unwrap())).collect();
    recursive_sub_sequences(base_vector)
}

fn recursive_sub_sequences(vec: Vec<u8>) -> Vec<Vec<u8>> {
    assert!(!vec.is_empty());
    if vec.len() == 1 {
        return vec![vec];
    }
    let mut return_vector: Vec<Vec<u8>> = Vec::new();
    for i in 0..vec.len() {
        let head = vec![vec[i]];
        let mut tail = vec.clone();
        tail.remove(i);
        for vector in recursive_sub_sequences(tail) {
            return_vector.push([head.clone(), vector.clone()].concat());
        }
    }
    return_vector
}
/// gives a random vec that's valid to create a pile from, given a set length.
pub fn random_vec(lenght: usize) -> Vec<u8> {
    let mut vec: Vec<u8> = (1u8..(u8::try_from(lenght + 1).unwrap())).collect();
    assert!(vec.len() == lenght);
    assert!(correct_sequence(&vec));
    assert!(!vec.contains(&0u8));
    vec.shuffle(&mut thread_rng());
    vec
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_sequence_true() {
        assert!(correct_sequence(&vec![1, 2, 3, 4, 5]))
    }
    #[test]
    fn correct_sequence_false() {
        assert!(!correct_sequence(&vec![1, 2, 4]));
        assert!(!correct_sequence(&vec![3, 4, 5, 6]));
    }
    #[test]
    fn test_all_sequences() {
        let sequences = all_sequences(3);
        let expected_sequences: Vec<Vec<u8>> = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        for sequence in &sequences {
            assert!(
                expected_sequences.contains(sequence),
                " got {:?}, expected {:?}",
                &sequences,
                &expected_sequences
            );
        }
        for sequence in &expected_sequences {
            assert!(
                sequences.contains(sequence),
                "got {:?}, expected {:?}",
                &sequences,
                &expected_sequences
            );
        }
        assert_eq!(sequences.len(), expected_sequences.len());
    }
    #[test]
    fn random_works() {
        let lenght = 5;
        let vec: Vec<u8> = random_vec(lenght);
        assert!(vec.len() == lenght);
        assert!(correct_sequence(&vec));
        assert!(!vec.contains(&0u8));
    }
}

#![cfg(test)]

// problem1 basic tests

use problem1;

#[test]
fn test_sum() {
    let array = [1,2,3,4,5];
    assert_eq!(problem1::sum(&array), 15);
}

#[test]
fn test_dedup() {
    let vs = vec![1,2,3,4];
    assert_eq!(problem1::dedup(&vs), vec![1,2,3,4]);

    let vs = vec![1,1,2,3,3,3,3,3,4,5,5,6,7,7];
    assert_eq!(problem1::dedup(&vs), vec![1,2,3,4,5,6,7]);

    let vs = vec![];
    assert_eq!(problem1::dedup(&vs), vec![]);
}

fn odd_predicate(x: i32) -> bool {
    (x % 2) == 1
}

#[test]
fn test_filter() {
    let vs = vec![1,2,3,4,5];
    assert_eq!(problem1::filter(&vs, &odd_predicate), vec![1,3,5]);
}

// problem2 basic tests

use problem2;

#[test]
#[should_panic]
fn test_asserts() {
    let mat1 = vec![vec![1f32,2f32,3f32], vec![2f32,3f32,4f32]];
    let mat2 = vec![vec![1f32], vec![2f32]];
    problem2::mat_mult(&mat1, &mat2);
}

#[test]
fn test_mult() {
    let mat1 = vec![vec![1f32,2f32,3f32],vec![4f32,5f32,6f32]];
    let mat2 = vec![vec![1f32,2f32,2f32,2f32],vec![3f32,4f32,2f32,2f32],vec![5f32,6f32,2f32,2f32]];
    assert_eq!(problem2::mat_mult(&mat1, &mat2), vec![vec![22f32, 28f32, 12f32, 12f32], vec![49f32, 64f32, 30f32, 30f32]]);
}

// problem3 basic tests

use problem3;

#[test]
fn test_primes() {
    assert_eq!(problem3::sieve(7), vec![2,3,5]);
}

use problem4::{hanoi, Peg};

#[test]
fn test_hanoi_3_discs() {
    let a: Peg = Peg::A;
    let b: Peg = Peg::B;
    let c: Peg = Peg::C;
    assert_eq!(hanoi(3, a, b, c), vec![
        (a, c),
        (a, b),
        (c, b),
        (a, c),
        (b, a),
        (b, c),
        (a, c)
    ])
}

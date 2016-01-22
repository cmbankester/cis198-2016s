#![cfg(test)]

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

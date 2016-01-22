// problem1.rs

/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut collector = 0i32;
    for el in slice {
        collector = collector + el;
    }
    collector
}

/// Deduplicate items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, in order.
pub fn dedup(vector: &Vec<i32>) -> Vec<i32> {
    let mut seen: Vec<i32> = Vec::with_capacity(vector.len());
    for el in vector {
        if !seen.contains(el) {
            seen.push(*el);
        }
    }
    seen
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut filtered: Vec<i32> = Vec::with_capacity(vs.len());
    for el in vs {
        if pred(*el) {
            filtered.push(*el);
        }
    }
    filtered
}

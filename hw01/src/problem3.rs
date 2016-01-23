/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut checked = Vec::new();
    for i in 2..n {
        if !checked.contains(&i) {
            primes.push(i);
            let mut j = i * i;
            while j < n {
                checked.push(j);
                j += i;
            }
        }
    }
    primes
}

fn main() {
    let mut sieve = [true; 10_000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10_000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);
}

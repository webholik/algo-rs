// Sieve of Eratosthenes

fn primes_upto(size: usize) -> Vec<usize> {
    let mut sieve = vec![0; size + 1];
    let mut primes = Vec::new();

    for i in 2..(size + 1) {
        if sieve[i] == 1 {
            continue;
        }
        primes.push(i);
        let mut j = i + i;
        while j < size + 1 {
            sieve[j] = 1;
            j += i;
        }
    }

    primes
}

#[test]
fn primes_upto_test() {
    macro_rules! is_prime {
        ($arr:ident, $val:expr) => {
            assert_eq!(Ok($val), $arr.binary_search(&$val).map(|index| $arr[index]));
        };
    }
    let primes = primes_upto(100);
    is_prime!(primes, 2);
    is_prime!(primes, 3);
    is_prime!(primes, 97);
}
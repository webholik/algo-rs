fn euclid(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    if b == 0 {
        a
    } else {
        euclid(b, a % b)
    }
}

fn euclid_iterative(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

#[test]
fn euclid_test() {
    assert_eq!(euclid(4, 16), 4);
    assert_eq!(euclid(0, 19), 19);
    assert_eq!(euclid(-4, 16), 4);
    assert_eq!(euclid_iterative(4, 16), 4);
    assert_eq!(euclid_iterative(0, 19), 19);
    assert_eq!(euclid_iterative(-4, 16), 4);
}

fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    let asign = if a.is_positive() { 1 } else { -1 };
    let bsign = if b.is_positive() { 1 } else { -1 };
    let a = a.abs();
    let b = b.abs();

    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_euclid(b, a % b);
        (g, asign * y, bsign * (x - (a / b) * y))
    }
}

#[test]
fn extended_euclid_test() {
    macro_rules! _test {
        ($a:expr, $b:expr) => {
            let (g, x, y) = extended_euclid($a, $b);
            assert_eq!(g, euclid($a, $b));
            assert_eq!(x * $a + y * $b, g);
        };
    }

    _test!(9, 81);
    _test!(-76, 43);
    _test!(-4, 16);
    _test!(0, 0);
}

fn multiplicative_inverse(a: i64, n: i64) -> i64 {
    let (g, mut x, _) = extended_euclid(a, n);
    // Assume n is prime
    assert_eq!(g, 1);
    if x < 0 {
        x = n - ((-x) % n);
    }

    x
}

#[test]
fn multiplicative_inverse_test() {
    assert_eq!(multiplicative_inverse(5, 11), 9);
}

fn solve_modular(a: i64, b: i64, n: i64) -> Option<Vec<i64>> {
    let (d, x, _) = extended_euclid(a, n);
    if b % d != 0 {
        return None;
    }

    let mut out = Vec::new();
    let x0 = (x * b / d) % n;

    for i in 0..d {
        let mut xtemp = (x0 + i * (n / d)) % n;
        if xtemp < 0 {
            xtemp = n - (-xtemp % n);
        }

        out.push(xtemp);
    }

    Some(out)
}

#[test]
fn solve_modular_test() {
    assert_eq!(solve_modular(2, 3, 5), Some(vec![4]));
    assert_eq!(solve_modular(14, 30, 100), Some(vec![95, 45]));
    assert_eq!(solve_modular(3, 1, 16), Some(vec![11]));
    assert_eq!(solve_modular(4, 1, 16), None);
}


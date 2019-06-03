fn robin_karp(input: &[u8], pattern: &[u8]) -> usize {
    let m = pattern.len();
    let n = input.len();
    if n < m {
        return n;
    }
    let mut p = 0usize;
    let mut t = 0usize;
    for i in 0..m {
        p += pattern[i] as usize;
        t += input[i] as usize;
    }

    for i in 0..(n - m + 1) {
        if p == t {
            let mut flag = true;
            for j in 0..m {
                if pattern[j] != input[i + j] {
                    flag = false;
                    break;
                }
            }
            if flag == true {
                return i;
            }
        }
        t = t - input[i] as usize + input[i + m] as usize;
    }

    return n;
}

#[test]
fn robin_karp_test() {
    assert_eq!(robin_karp(b"abdabc", b"bda"), 1);
}
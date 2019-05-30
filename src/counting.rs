fn counting_sort(input: &[usize], mut max: usize) -> Vec<usize> {
    max += 1;
    let mut c = vec![0; max];
    let mut b = vec![0; input.len()];
    for &i in input.iter() {
        c[i] += 1;
    }

    for i in 1..max {
        c[i] += c[i - 1];
    }

    for &i in input.iter().rev() {
        b[c[i] - 1] = i;
        c[i] -= 1;
    }

    b

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn counting() {
        let arr = [3, 2, 9, 11, 23, 7, 8];
        assert_eq!(counting_sort(&arr, 23), vec![2, 3, 7, 8, 9, 11, 23]);
    }
}
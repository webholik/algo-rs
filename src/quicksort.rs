fn partition(arr: &mut [i64]) -> usize {
    let len = arr.len();
    if len == 0 {
        return 0;
    }
    let key = arr[len - 1];
    let mut j = len - 1;
    let mut i = 0;
    while i < j {
        if arr[i] > key {
            j -= 1;
            arr.swap(i, j);
        } else {
            i += 1;
        }
    }

    arr.swap(j, len - 1);

    return j;
}

fn quicksort(arr: &mut [i64]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let pivot = partition(arr);
    quicksort(&mut arr[0..pivot]);
    quicksort(&mut arr[pivot + 1..len]);
}

#[test]
fn partition_test() {
    let mut arr = [1, 2, 3, 4, 9, 6];
    assert_eq!(partition(&mut arr), 4);
    assert_eq!(arr, [1, 2, 3, 4, 6, 9]);
}

#[test]
fn quicksort_test() {
    let mut arr = [8, 3, 2, 1, 0, 33, 2, 9];
    quicksort(&mut arr);
    assert_eq!(arr, [0, 1, 2, 2, 3, 8, 9, 33]);
}
// Find maximum subarray sum using Divide and Conquer algorithm

pub fn max_subarray_mid(arr: &[i64], low: usize, high: usize) -> (usize, usize, i64) {
    let mid = (low + high) / 2;
    let mut sum_left = arr[mid];
    let mut sum_right = 0;
    let mut sum = arr[mid];

    let mut index_left = mid;
    let mut index_right = mid;

    for (i, a) in arr[low..mid].iter().rev().enumerate() {
        sum += a;
        if sum > sum_left {
            sum_left = sum;
            index_left = mid - i - 1;
        }
    }

    sum = 0;

    for i in (mid + 1)..(high + 1) {
        sum += arr[i];
        if sum > sum_right {
            sum_right = sum;
            index_right = i;
        }
    }

    (index_left, index_right, sum_left + sum_right)
}

pub fn max_subarray(arr: &[i64], low: usize, high: usize) -> (usize, usize, i64) {
    if low == high {
        (low, high, arr[low])
    } else {
        let mid = (low + high) / 2;
        let (low_left, high_left, sum_left) = max_subarray(arr, low, mid);
        let (low_right, high_right, sum_right) = max_subarray(arr, mid + 1, high);
        let (low_mid, high_mid, sum_mid) = max_subarray_mid(arr, low, high);

        if sum_left > sum_right {
            if sum_left > sum_mid {
                return (low_left, high_left, sum_left);
            } else {
                return (low_mid, high_mid, sum_mid);
            }
        } else {
            if sum_right > sum_mid {
                return (low_right, high_right, sum_right);
            } else {
                return (low_mid, high_mid, sum_mid);
            }
        }
    }
}
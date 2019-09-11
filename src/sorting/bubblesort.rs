fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..(arr.len()-i-1) {
            if arr[j] > arr[j+1] {
                let temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
}


#[test]
fn bubble_test() {
    let mut arr = [4, 9, 2, 5, 3];
    let sorted = [2, 3, 4, 5, 9];
    bubble_sort(&mut arr);
    for i in 1..arr.len() {
        assert_eq!(arr[i], sorted[i], "different sort at pos {}", i);
    }
}

fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[i] > arr[j] {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }
}


#[test]
fn bubble_test() {
    let mut arr = [1, 9, 4, 5, 3];
    let sorted = [1, 3, 4, 5, 9];
    bubble_sort(&mut arr);
    for i in 1..arr.len() {
        assert_eq!(arr[i], sorted[i], "different sort at pos {}", i);
    }
}

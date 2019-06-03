fn heapify(arr: &mut [i64]) {
    for i in 1..arr.len() {
        macro_rules! get_parent {
            ($i:ident) => {
                if $i % 2 == 0 {
                    $i / 2 - 1
                } else {
                    $i / 2
                }

            };
        }
        // let mut parent = if i % 2 == 0 { i / 2 - 1 } else { i / 2 };
        let mut parent = get_parent!(i);
        let mut child = i;

        loop {
            if arr[child] > arr[parent] {
                arr.swap(child, parent);
            }

            if parent == 0 {
                break;
            }

            child = parent;
            parent = get_parent!(parent);
        }
    }
}

fn heap_sort(arr: &mut [i64]) {
    heapify(arr);
    let max_index = arr.len() - 1;
    for i in 0..max_index {
        arr.swap(0, max_index - i);
        heapify(&mut arr[0..max_index - i]);
    }
}

fn push_down(arr: &mut [i64]) {
    arr.swap(0, arr.len() - 1);
    let mut i = 0;
    loop {
        let child1 = i * 2 + 1;
        if child1 >= arr.len() - 1 {
            break;
        }
        let child2 = child1 + 1;
        if child2 >= arr.len() - 1 {
            if arr[child1] > arr[i] {
                arr.swap(child1, i);
            }

            break;
        }

        let bigger = if arr[child1] > arr[child2] {
            child1
        } else {
            child2
        };

        if arr[i] >= arr[bigger] {
            break;
        } else {
            arr.swap(i, bigger);
            i = bigger;
        }
    }
}

fn heap_sort2(arr: &mut [i64]) {
    heapify(arr);
    let max_index = arr.len() - 1;
    for i in 0..max_index {
        push_down(&mut arr[0..max_index - i + 1]);
    }
}

#[test]
fn heapify_test() {
    let mut arr = [4, 9, 12, 9, 2, 3, 4];
    heapify(&mut arr);
    assert_eq!(arr[..], [12, 9, 9, 4, 2, 3, 4]);
}

#[test]
fn heapsort_test() {
    let mut arr = [5, 9, 12, 8, 2, 1, 0];
    heap_sort(&mut arr[..]);
    assert_eq!(arr[..], [0, 1, 2, 5, 8, 9, 12]);
}

#[test]
fn push_down_test() {
    let mut arr = [5, 9, 12, 8, 2, 1, 0];
    heapify(&mut arr);
    push_down(&mut arr);
    assert_eq!(arr[..], [9, 8, 1, 5, 2, 0, 12]);
}

#[test]
fn heap_sort2_test() {
    let mut arr = [5, 9, 12, 8, 2, 1, 0];
    heap_sort2(&mut arr);
    assert_eq!(arr[..], [0, 1, 2, 5, 8, 9, 12]);
}
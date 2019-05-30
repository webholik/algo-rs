#![allow(unused)]

mod counting;
mod heapsort;
mod quicksort;
fn main() {
    println!("Hello, world!");
    let mut arr = [1, 5, 3, 2, 0];
    for i in 0..5 {
        for j in i..5 {
            if arr[i] > arr[j] {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }

    // for elem in arr.iter() {
    //     println!("{}", elem);
    // }

    let mut arr = vec![19, 6, 2, 3, 0, 4];
    bubble_sort(&mut arr);
    myprint(&arr);
    let x = arr.get(4);
    match x {
        Some(i) => println!("Got back {}", i),
        None => println!("Got back nothing"),
    }
}

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

fn myprint(x: &Vec<i32>) {
    let mut s = String::new();
    for i in x {
        s.push_str(&(i.to_string() + " "));
    }
    println!("{}", s);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_bubble() {
        let mut arr = [1, 9, 4, 5, 3];
        let sorted = [1, 3, 4, 5, 9];
        bubble_sort(&mut arr);
        for i in 1..arr.len() {
            assert_eq!(arr[i], sorted[i], "different sort at pos {}", i);
        }
    }
}


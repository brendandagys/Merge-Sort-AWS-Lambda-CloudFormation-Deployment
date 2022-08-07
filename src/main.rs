use helpers::print_time;
use rand::{thread_rng, Rng};

mod helpers;

const MAX_RANDOM_NUMBER: i32 = 512;
const NUMBER_COUNT: usize = 16;

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;

    let mut merged: Vec<i32> = Vec::with_capacity(left.len() + right.len());

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1
        } else {
            merged.push(right[j]);
            j += 1
        }
    }

    // 1 - two statements
    // 2 - if-else inside `append()`
    // 3 - `match` inside `append()`
    // let mut leftover = if i < left.len() {
    //     left[i..left.len()].to_vec()
    // } else {
    //     right[j..right.len()].to_vec()
    // };

    // merged.append(&mut leftover);

    merged.append(&mut match i < left.len() {
        true => left[i..left.len()].to_vec(),
        false => right[j..right.len()].to_vec(),
    });

    merged
}

fn merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() == 1 {
        arr.to_vec()
    } else {
        let middle = arr.len() / 2;
        println!("Middle: {middle}");

        let left = merge_sort(&arr[0..middle]);
        let right = merge_sort(&arr[middle..]);

        return merge(&left, &right);
    }
}

fn main() {
    print_time("STARTING");

    let mut rng = thread_rng();

    let mut arr: [i32; NUMBER_COUNT] = [-1; NUMBER_COUNT];
    for i in 0..NUMBER_COUNT {
        arr[i] = rng.gen_range(1..MAX_RANDOM_NUMBER);
    }

    println!("NUMBERS: {:?}", arr);

    let sorted_numbers = merge_sort(&arr);
    println!("{:?}", sorted_numbers);

    print_time("SORTED!");
}

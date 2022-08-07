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

    let mut leftover = if i < left.len() {
        left[i..left.len()].to_vec()
    } else {
        right[j..right.len()].to_vec()
    };

    merged.append(&mut leftover);

    merged
}

fn merge_sort(vec: Vec<i32>) -> Vec<i32> {
    if vec.len() == 1 {
        vec
    } else {
        let middle = vec.len() / 2;
        println!("Middle: {middle}");

        let left = merge_sort(vec[0..middle].to_vec());
        let right = merge_sort(vec[middle..].to_vec());

        return merge(&left, &right);
    }
}

fn main() {
    print_time("STARTING");

    let mut rng = thread_rng();
    let mut numbers: Vec<i32> = Vec::with_capacity(NUMBER_COUNT);

    for _x in 0..numbers.capacity() {
        numbers.push(rng.gen_range(1..MAX_RANDOM_NUMBER))
    }

    println!("NUMBERS: {:?}", numbers);

    let sorted_numbers = merge_sort(numbers);
    println!("{:?}", sorted_numbers);

    print_time("SORTED!");
}

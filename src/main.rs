use chrono::{DateTime, Utc};
use std::ops::Range;

use rand::{thread_rng, Rng};

const NUMBER_COUNT: usize = 100;
const MAX_RANDOM_NUMBER: i32 = 512;

fn print_time(prefix: &str) -> DateTime<Utc> {
    let now = Utc::now();
    println!("{prefix}: {} {}", now.date(), now.time());
    now
}

fn main() {
    let start_time = print_time("STARTING");

    let mut rng = thread_rng();

    let mut numbers: Vec<i32> = Vec::with_capacity(NUMBER_COUNT);

    for x in (Range {
        start: 0,
        end: numbers.capacity(),
    }) {
        numbers.insert(x, rng.gen_range(1..MAX_RANDOM_NUMBER))
    }

    println!("NUMBERS: {:?}", numbers);

    fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
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

        let left_leftover = &mut left[i..left.len()].to_vec();
        let right_leftover = &mut right[j..right.len()].to_vec();

        merged.append(match i < left.len() {
            true => left_leftover,
            false => right_leftover,
        });

        // if i < left.len() {
        //     merged.append(&mut left[i..left.len()].to_vec());
        // } else {
        //     merged.append(&mut right[j..right.len()].to_vec());
        // }

        merged
    }

    fn merge_sort(vec: Vec<i32>) -> Vec<i32> {
        if vec.len() == 1 {
            vec
        } else {
            let middle = vec.len() / 2;

            let left = merge_sort(vec[0..middle].to_vec());
            let right = merge_sort(vec[middle..].to_vec());

            let merged = merge(left, right);

            merged
        }
    }

    let sorted_numbers = merge_sort(numbers);
    println!("{:?}", sorted_numbers);

    let end_time = print_time("SORTED!");
}

mod helpers;

use helpers::print_time;
use lambda_runtime;
use log;
use simple_logger;

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    // let mut i = 0; let mut j = 0;

    let (mut i, mut j) = (0, 0);

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

    // IF I HAVE TIME...
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

async fn my_handler(
    e: lambda_runtime::LambdaEvent<Vec<i32>>,
) -> Result<Vec<i32>, lambda_runtime::Error> {
    print_time("HANDLER STARTING!");

    println!("NUMBERS: {:?}", &e.payload);

    let sorted_numbers = merge_sort(&e.payload);
    println!("{:?}", sorted_numbers);

    print_time("HANDLER ENDING! Sorted!");

    return Ok(sorted_numbers);
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    print_time("MAIN STARTING");
    simple_logger::init_with_level(log::Level::Info)?;

    lambda_runtime::run(lambda_runtime::service_fn(my_handler)).await?;

    print_time("MAIN ENDING!");

    Ok(())
}

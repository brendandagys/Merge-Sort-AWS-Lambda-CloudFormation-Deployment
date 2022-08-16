mod helpers;
mod types;

use crate::types::{CustomOutput, ParsedRequestBody};
use helpers::print_time;
use lambda_runtime;
use log;
use simple_logger;
use types::ApiGatewayEvent;

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

async fn handler(
    e: lambda_runtime::LambdaEvent<ApiGatewayEvent>,
) -> Result<CustomOutput, lambda_runtime::Error> {
    print_time("HANDLER STARTING!");
    // println!("{:?}", e);

    let json_numbers = serde_json::from_str::<ParsedRequestBody>(&e.payload.body)
        .expect("Could not parse string into JSON!");

    let sorted_numbers = merge_sort(&json_numbers.numbers);
    println!("{:?}", sorted_numbers);

    print_time("HANDLER ENDING! Sorted!");

    let response = CustomOutput { sorted_numbers };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    print_time("MAIN STARTING");
    simple_logger::init_with_level(log::Level::Info)?;

    lambda_runtime::run(lambda_runtime::service_fn(handler)).await?;
    print_time("MAIN ENDING!");

    Ok(())
}

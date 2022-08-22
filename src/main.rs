mod helpers;
mod types;

use crate::types::{CustomOutput, ParsedRequestBody};
use helpers::print_time;
use lambda_runtime;
use log;
use simple_logger;
use types::ApiGatewayEvent;

fn merge_sorted_arrays(left: &[i32], right: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);

    let mut merged = Vec::with_capacity(left.len() + right.len());

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1
        } else {
            merged.push(right[j]);
            j += 1
        }
    }

    merged.append(&mut match i < left.len() {
        true => left[i..].to_vec(),
        false => right[j..].to_vec(),
    });

    merged
}

fn merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() == 1 {
        return arr.to_vec();
    }

    let middle = arr.len() / 2;
    println!("Middle: {middle}");

    let left = merge_sort(&arr[..middle]);
    let right = merge_sort(&arr[middle..]);

    merge_sorted_arrays(&left, &right)
}

async fn handler(
    e: lambda_runtime::LambdaEvent<ApiGatewayEvent>,
) -> Result<CustomOutput, lambda_runtime::Error> {
    print_time("HANDLER STARTING!");
    println!("{:?}", e);

    let request_body = serde_json::from_str::<ParsedRequestBody>(&e.payload.body)
        .expect("Could not parse string into JSON!");

    println!("NAME: {}", request_body.name);

    let sorted_numbers = merge_sort(&request_body.numbers);
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
